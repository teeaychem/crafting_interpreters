// use crate::interpreter::ast::expression::Basic;

// use super::EvalErr;

// impl Basic {
//     pub fn to_boolean(self) -> Result<Self, EvalErr> {
//         match self {
//             Basic::Nil => Ok(Basic::from(false)),

//             Basic::Boolean { .. } => Ok(self),

//             _ => Ok(Basic::from(true)),
//         }
//     }

//     pub fn to_numeric(self) -> Result<Self, EvalErr> {
//         match self {
//             Basic::Nil => Err(EvalErr::InvalidConversion),

//             Basic::Boolean { b } => Err(EvalErr::InvalidConversion),

//             Self::String { s } => match s.parse::<f64>() {
//                 Ok(v) => Ok(Basic::from(v)),

//                 Err(_) => Err(EvalErr::InvalidConversion),
//             },

//             Self::Numeric { .. } => Ok(self),
//         }
//     }

//     pub fn to_string(self) -> Result<Self, EvalErr> {
//         let value = match self {
//             Basic::Nil => return Err(EvalErr::InvalidConversion),

//             Basic::Boolean { b } => match b {
//                 true => Basic::from("true"),
//                 false => Basic::from("false"),
//             },

//             Self::String { .. } => self,

//             Self::Numeric { n } => Basic::from(n.to_string()),
//         };

//         Ok(value)
//     }
// }
