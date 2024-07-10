pub mod google {
    #[path = ""]
    pub mod cloud {
        #[path = ""]
        pub mod aiplatform {
            #[path = "google.cloud.aiplatform.v1.rs"]
            pub mod v1;
        }
    }

    #[path = "google.api.rs"]
    pub mod api;

    #[path = "google.r#type.rs"]
    pub mod r#type;
}
