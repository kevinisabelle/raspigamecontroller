// rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemImpl};

#[proc_macro_attribute]
pub fn gatt_characteristic(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input impl block.
    let impl_block = parse_macro_input!(item as ItemImpl);

    // Extract self type and original items.
    let self_ty = &impl_block.self_ty;
    let orig_items = &impl_block.items;

    // Define the extra methods to add.
    let methods = quote! {
        #[zbus(property)]
        fn get_flags(&self) -> Vec<String> {
            let flags = self.0.lock().unwrap().base.flags.clone();
            println!("Get Flags: {:?} from { }", flags, self.0.lock().unwrap().base.uuid.clone());
            flags
        }

        #[zbus(property)]
        fn get_uuid(&self) -> String {
            let uuid = self.0.lock().unwrap().base.uuid.clone();
            println!("Get UUID: {:?} from { }", uuid, self.0.lock().unwrap().base.uuid.clone());
            uuid
        }

        #[zbus(property)]
        fn get_service(&self) -> String {
            let service = self.0.lock().unwrap().base.service.clone();
            println!("Get Service: {:?} from { }", service, self.0.lock().unwrap().base.uuid.clone());
            service
        }

        #[zbus(property)]
        fn get_descriptors(&self) -> Vec<String> {
            let descriptors = self.0.lock().unwrap().base.descriptors.clone();
            println!("Get Descriptors: {:?} from { }", descriptors, self.0.lock().unwrap().base.uuid.clone());
            descriptors
        }
    };

    // Generate the output with proper iteration.
    let output = quote! {
        #[interface(name = "org.bluez.GattCharacteristic1")]
        impl #self_ty {
            // Original items.
            #(#orig_items)*

            #methods
        }
    };

    TokenStream::from(output)
}

#[proc_macro_attribute]
pub fn gatt_descriptor(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input impl block.
    let impl_block = parse_macro_input!(item as ItemImpl);
    // Extract self type and original items.

    let self_ty = &impl_block.self_ty;
    let orig_items = &impl_block.items;

    // Define the extra methods to add.
    let methods = quote! {
        #[zbus(property)]
        fn get_flags(&self) -> Vec<String> {
            self.0.lock().unwrap().base.flags.clone()
        }

        #[zbus(property)]
        fn get_uuid(&self) -> String {
            self.0.lock().unwrap().base.uuid.clone()
        }

        #[zbus(property)]
        fn get_characteristic(&self) -> String {
            self.0.lock().unwrap().base.characteristic.clone()
        }

        #[zbus(property)]
        fn get_path(&self) -> String {
            self.0.lock().unwrap().base.path.clone()
        }
    };

    let output = quote! {
        #[interface(name = "org.bluez.GattDescriptor1")]
        impl #self_ty {
            // Original items.
            #(#orig_items)*

            #methods
        }
    };

    TokenStream::from(output)
}

#[proc_macro_attribute]
pub fn gatt_service(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input impl block.
    let impl_block = parse_macro_input!(item as ItemImpl);
    let self_ty = &impl_block.self_ty;
    let orig_items = &impl_block.items;

    // Define the extra methods to add.
    let methods = quote! {
        #[zbus(property)]
        fn get_primary(&self) -> bool {
            self.0.lock().unwrap().base.primary
        }

        #[zbus(property)]
        fn get_uuid(&self) -> String {
            self.0.lock().unwrap().base.uuid.clone()
        }

        #[zbus(property)]
        fn get_characteristics(&self) -> Vec<String> {
            self.0.lock().unwrap().base.characteristics.clone()
        }
    };

    let output = quote! {
        #[interface(name = "org.bluez.GattService1")]
        impl #self_ty {
            // Original items.
            #(#orig_items)*
            #methods
        }
    };

    TokenStream::from(output)
}
