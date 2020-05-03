 mod outermost {
    pub fn middle_function() {}

    pub fn middle_secret_function() {}

    pub mod inside {
        pub fn inner_function() {
            //::outermost::middle_function();
            // cannt not find 'outermost' in '{{root}}
        }
        pub fn secret_function() {}
    }
}

fn tey_me() {
    outermost::middle_function();
    outermost::middle_secret_function();
    outermost::inside::inner_function();
    outermost::inside::secret_function();
}
