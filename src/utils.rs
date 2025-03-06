// // app utils

// pub fn build_map<T>(pairs: Vec<(&str, T)>) -> std::collections::HashMap<&str, T> {
//   let mut new_map = std::collections::HashMap::new();
//   for (k, v) in pairs {
//       if new_map.contains_key(k) {
//           panic!("Keys must be unique : {k}");
//       }
//       new_map.insert(k, v);
//   }
//   return new_map;
// }