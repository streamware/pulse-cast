## Pulse Cast
Pulse Cast is a Rust-based service designed to receive and process various Pulsar messages and then send Firebase notifications to devices. The service currently supports Firebase Cloud Messaging and is intended to expand its capabilities to include sending emails and other integrations in the future.

### Technologies Used
- 🦀 Rust: Built with performance and safety in mind.
- ⚙️ Tokio: An asynchronous runtime for the Rust programming language.
- 🌐 Axum: Utilizes the Axum web framework for robust and scalable web handling.
- 🗃️ Diesel ORM: Ensures type-safe database interactions.
- 📡 Pulsar: Integration for receiving and processing messages.
- 🔥 Firebase Cloud Messaging: Sends notifications to devices.

### Advantages
- 🟢 Resource Efficient: Unlike alternatives like Novu, which require significant resources (at least 8GB RAM for local development and 32GB-64GB for production), this service can run on minimal hardware (32MB for testing and 2GB for production).
- 🛠️ Customizable: Easily modify any part of the service to fit your specific needs.

## Why This Project?
The motivation behind creating Pulse Cast stems from the need for a more efficient, customizable, and resource-friendly notification system. Existing solutions like Novu, while powerful, come with significant overhead in terms of system resources and limitations in the community version. This project aims to address these issues by providing a lightweight alternative that can be easily tailored to specific needs.

By utilizing Rust, Axum, and Diesel ORM, the service ensures high performance and safety. The integration with Pulsar allows for scalable and reliable message processing, while the initial support for Firebase Cloud Messaging sets the stage for expanding to other notification channels. This flexibility and efficiency make it an ideal solution for developers looking for a robust notification system without the heavy resource requirements.

Thank you for checking out Pulse Cast! Your feedback and contributions are highly appreciated.
