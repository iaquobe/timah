use std::ops::Add;

pub struct Times {
    pub total  :i32,
    pub split  :i32,
    pub day    :i32,
    pub week   :i32,
    pub month  :i32,
}

impl Add for Times {
    type Output = Self; 

    fn add(self, rhs: Self) -> Self {
        Self { 
            split:  self.split  + rhs.split,
            day:    self.day    + rhs.day,
            week:   self.week   + rhs.week,
            month:  self.month  + rhs.month,
            total:  self.total  + rhs.total,
        }
    }
}

impl Default for Times {
    fn default() -> Self {
        Self { total: 0, split: 0, day: 0, week: 0, month: 0 }
    }
}
