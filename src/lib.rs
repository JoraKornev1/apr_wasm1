extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;



#[wasm_bindgen]
pub fn apr(amount: f64, apr: f64, days: f64, period: f64 ) -> f64 {
    let apr_per_day = apr / 365.0; 
    let mut am = amount; 
    let mut reward_per_day = apr_per_day * amount; 
    let mut day = 0.0;
    let remains = period % days;
  
    if remains == 0.0 {
        while day < period  {
            am = am + reward_per_day * days; 
            reward_per_day = am * apr_per_day;
            day = day + days;
                        
        }
  
    } else {
        while day < (period - days)  {
            am = am + reward_per_day * days; 
            reward_per_day = am * apr_per_day;
            day = day + days;
                      
        }
        am = am + reward_per_day * remains;
        
    }   
       
    am
  }