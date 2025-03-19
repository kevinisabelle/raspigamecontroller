// rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemImpl};

#[proc_macro_attribute]
pub fn gatt_chrc_properties(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input impl block.
    let mut impl_block = parse_macro_input!(item as ItemImpl);

    // Generate the methods to be appended.
    let properties_methods = quote! {
        #[zbus(property)]
        fn get_flags(&self) -> Vec<String> {
            self.0.lock().unwrap().base.flags.clone()
        }

        #[zbus(property)]
        fn get_uuid(&self) -> String {
            self.0.lock().unwrap().base.uuid.clone()
        }

        #[zbus(property)]
        fn get_service(&self) -> String {
            self.0.lock().unwrap().base.service.clone()
        }

        #[zbus(property)]
        fn get_descriptors(&self) -> Vec<String> {
            self.0.lock().unwrap().base.descriptors.clone()
        }
    };

    // Wrap the methods in a dummy impl block and parse it.
    let dummy_impl = syn::parse2::<ItemImpl>(quote! {
        impl Dummy {
            #properties_methods
        }
    })
    .expect("failed to parse dummy impl");

    // Extract the methods from the dummy impl block.
    let methods = dummy_impl.items;

    // Append the methods to the original impl block.
    impl_block.items.extend(methods);

    // Return the final token stream.
    TokenStream::from(quote! {
        #impl_block
    })
}

#[proc_macro_attribute]
pub fn gatt_desc_properties(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input impl block.
    let mut impl_block = parse_macro_input!(item as ItemImpl);

    // Generate the methods to be appended.
    let properties_methods = quote! {
        #[zbus::zbus(property)]
        fn get_flags(&self) -> Vec<String> {
            self.0.lock().unwrap().base.flags.clone()
        }

        #[zbus::zbus(property)]
        fn get_uuid(&self) -> String {
            self.0.lock().unwrap().base.uuid.clone()
        }

        #[zbus::zbus(property)]
        fn get_characteristic(&self) -> String {
            self.0.lock().unwrap().base.characteristic.clone()
        }

        #[zbus::zbus(property)]
        fn get_path(&self) -> String {
            self.0.lock().unwrap().base.path.clone()
        }
    };

    // Wrap the methods in a dummy impl block and parse it.
    let dummy_impl = syn::parse2::<ItemImpl>(quote! {
        impl Dummy {
            #properties_methods
        }
    })
    .expect("failed to parse dummy impl");

    // Extract the methods from the dummy impl block.
    let methods = dummy_impl.items;

    // Append the methods to the original impl block.
    impl_block.items.extend(methods);

    // Return the final token stream.
    TokenStream::from(quote! {
        #impl_block
    })
}

#[proc_macro_attribute]
pub fn gatt_service_properties(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input impl block.
    let mut impl_block = parse_macro_input!(item as ItemImpl);

    // Generate the methods to be appended.
    let properties_methods = quote! {
        #[zbus::zbus(property)]
    fn get_primary(&self) -> bool {
        self.0.lock().unwrap().base.primary
    }

    #[zbus::zbus(property)]
    fn get_uuid(&self) -> String {
        self.0.lock().unwrap().base.uuid.clone()
    }

    #[zbus::zbus(property)]
    fn get_characteristics(&self) -> Vec<String> {
        self.0.lock().unwrap().base.characteristics.clone()
    }
    };

    // Wrap the methods in a dummy impl block and parse it.
    let dummy_impl = syn::parse2::<ItemImpl>(quote! {
        impl Dummy {
            #properties_methods
        }
    })
    .expect("failed to parse dummy impl");

    // Extract the methods from the dummy impl block.
    let methods = dummy_impl.items;

    // Append the methods to the original impl block.
    impl_block.items.extend(methods);

    // Return the final token stream.
    TokenStream::from(quote! {
        #impl_block
    })
}
