extern crate proc_macro;
extern crate syn;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(HashMapDelegate)]
pub fn hashmap_delegate(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &input.ident;

    let output = quote! {
        impl #name {
            fn new() -> Self {
                Self {
                    map: std::collections::HashMap::new(),
                }
            }

            fn with_capacity(capacity: usize) -> Self {
                Self {
                    map: std::collections::HashMap::with_capacity(capacity),
                }
            }

            fn capacity(&self) -> usize {
                self.map.capacity()
            }

            fn reserve(&mut self, additional: usize) {
                self.map.reserve(additional)
            }

            fn shrink_to_fit(&mut self) {
                self.map.shrink_to_fit()
            }

            fn clear(&mut self) {
                self.map.clear()
            }

            fn insert(&mut self, k: String, v: String) -> Option<String> {
                self.map.insert(k, v)
            }

            fn remove(&mut self, k: &String) -> Option<String> {
                self.map.remove(k)
            }

            fn get(&self, k: &String) -> Option<&String> {
                self.map.get(k)
            }

            fn get_mut(&mut self, k: &String) -> Option<&mut String> {
                self.map.get_mut(k)
            }

            fn contains_key(&self, k: &String) -> bool {
                self.map.contains_key(k)
            }

            fn len(&self) -> usize {
                self.map.len()
            }

            fn is_empty(&self) -> bool {
                self.map.is_empty()
            }

            fn drain(&mut self) -> std::collections::hash_map::Drain<String, String> {
                self.map.drain()
            }

            fn keys(&self) -> std::collections::hash_map::Keys<String, String> {
                self.map.keys()
            }

            fn values(&self) -> std::collections::hash_map::Values<String, String> {
                self.map.values()
            }

            fn iter(&self) -> std::collections::hash_map::Iter<String, String> {
                self.map.iter()
            }

            fn iter_mut(&mut self) -> std::collections::hash_map::IterMut<String, String> {
                self.map.iter_mut()
            }

            fn entry(&mut self, k: String) -> std::collections::hash_map::Entry<String, String> {
                self.map.entry(k)
            }

            fn get_key_value(&self, k:&String) -> Option<(&String, &String)> {
                self.map.get_key_value(k)
            }

            fn retain<F>(&mut self, f: F)
            where
                F: FnMut(&String, &mut String) -> bool,
            {
                self.map.retain(f)
            }

            fn extend<T>(&mut self, iter: T)
            where
                T: IntoIterator<Item = (String, String)>,
            {
                self.map.extend(iter)
            }

            fn clone(&self) -> Self {
                Self {
                    map: self.map.clone(),
                }
            }
        }
    };
    output.into()
}
