pub trait IInstallemnt {
    fn monlty_installment(&self) -> Vec<String> {
        let mut installments: Vec<String> = Vec::with_capacity(0);
        installments.push(String::from("read more"));
        installments
    }
}

pub struct CarInstallment {
    pub is_new: bool,
    pub created_year: u32,
    pub base_interest: f32,
    pub down_payment: u32,
    pub price: u32,
    pub tenor: u16,
}

fn sanitize_created_year(created_year: u32) -> Result<u32, &'static str> {
    let len = created_year.checked_ilog10().unwrap_or(0) + 1;
    if len == 4 {
        Ok(created_year)
    } else {
        Err("Invalid Created Year")
    }
}

fn sanitize_is_new(condition: String) -> Result<bool, &'static str> {
    match condition {
        _ if condition == "baru" => Ok(true),
        _ if condition == "bekas" => Ok(false),
        _ => Err("Invalid Condition"),
    }
}

fn sanititize_down_payment(
    down_payment: u32,
    is_new: bool,
    price: u32,
) -> Result<u32, &'static str> {
    let multiplier;
    if is_new {
        multiplier = 25. / 100.;
    } else {
        multiplier = 35. / 100.;
    }
    let min_dp = price as f32 * multiplier;
    if min_dp > down_payment as f32 {
        return Err("Minimum Down Payment");
    }
    Ok(down_payment)
}

fn sanitize_tenor(tenor: u16) -> Result<u16, &'static str> {
    if tenor > 6 || tenor < 1 {
        return Err("Tenor Range");
    }
    Ok(tenor)
}

impl CarInstallment {
    pub fn new(
        condition: String,
        created_year: u32,
        base_interest: f32,
        down_payment: u32,
        price: u32,
        tenor: u16,
    ) -> Result<Self, &'static str> {
        let is_new = sanitize_is_new(condition)?;
        let created_year = sanitize_created_year(created_year)?;
        let down_payment = sanititize_down_payment(down_payment, is_new, price)?;
        let tenor = sanitize_tenor(tenor)?;
        Ok(CarInstallment {
            is_new,
            created_year,
            base_interest,
            down_payment,
            price,
            tenor,
        })
    }
}

impl IInstallemnt for CarInstallment {
    fn monlty_installment(&self) -> Vec<String> {
        let mut base_interest = self.base_interest;
        let mut installments: Vec<String> = Vec::with_capacity(usize::from(self.tenor));
        let mut principle_aggregate = self.price as f32 - self.down_payment as f32;
        for year in 1..=self.tenor {
            let total_loan = principle_aggregate + (principle_aggregate * base_interest);
            let montly_installment =
                total_loan / (12. * (self.tenor as u32 - year as u32 + 1) as f32);
            let yearly_installment = montly_installment * 12.;
            principle_aggregate = total_loan - yearly_installment;
            installments.push(montly_installment.to_string());
            println!(
                "Tahun {} : Rp.{}/bulan, Suku Bunga: {}%",
                year, montly_installment, self.base_interest
            );
            if year % 2 == 0 {
                base_interest += 0.005;
                continue;
            }
            base_interest += 0.001;
        }
        installments
    }
}
