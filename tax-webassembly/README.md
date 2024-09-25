# Tax Calculator Web App with Rust and WebAssembly
=====================================================

## Summary
-----------

 We are build a simple tax calculator web application using Rust and WebAssembly. You'll learn the setup, coding, and deployment steps to get your app up and running.

## Highlights
--------------

* 🚀 **Introduction to WebAssembly and Rust basics**: Get familiar with the fundamentals of WebAssembly and Rust.
* 💻 **Project setup**: Create and configure a new Rust project from scratch.
* ✍️ **Writing Rust code for a tax calculator application**: Learn how to write efficient Rust code for your tax calculator app.
* 🛠️ **Building the project with wasm-pack**: Simplify the building and packaging process with wasm-pack.
* 🌐 **Setting up an HTML file to load the WebAssembly module**: Learn how to set up an HTML file to load your WebAssembly module.
* 🔍 **Testing the application using a simple HTTP server**: Test your app in a real-world environment with a simple HTTP server.
* 📝 **Importance of understanding Rust for WebAssembly integration**: Understand why Rust is essential for WebAssembly integration.

## Key Insights
----------------

### WebAssembly Overview

🌟 **WebAssembly (Wasm) is a binary instruction format** that allows high-performance applications in web browsers, enabling languages like Rust to be compiled for web use. This expands the capabilities of web applications beyond JavaScript.

### Rust and JavaScript Interaction

🔗 **wasm-bindgen facilitates seamless interaction** between Rust and JavaScript, allowing developers to leverage Rust's performance within web applications while managing UI with JavaScript.

### Project Configuration

📦 **Proper configuration in the Cargo.toml file** is crucial for building a Rust project aimed at WebAssembly, ensuring the output is a C-compatible dynamic library.

### Building with wasm-pack

⚙️ **Using wasm-pack simplifies the process** of building and packaging Rust code for the web, generating necessary files for integration with JavaScript.

### Testing Environment

🖥️ **Setting up a local HTTP server is essential** for testing web applications, as it mimics production environments and allows for real-time testing in browsers.

### Tax Calculation Logic

📊 **The tax calculator logic demonstrates practical Rust programming concepts** such as conditionals and data types, making the application both functional and educational.

### Learning Curve

🎓 **While prior programming knowledge is beneficial**, the simplicity of Rust code in this tutorial allows beginners to grasp concepts without extensive Rust experience.

## Running the Project
---------------------

### Step 1: Build the Project

We need to install wasm-pack to build wasm pack, run the following command in the terminal:
```bash
    cargo install wasm-pack
```

### Step 2: Build the Project

In the project root folder, run the following command in the terminal:
```bash
    wasm-pack build --target web
```
This command will create the necessary JavaScript and Wasm files in the `pkg` directory.

### Step 3: Install http-server

Run the following command to install http-server:
```bash
    npm install -g http-server
```

### Step 4: Start the Server

In the project root directory, run the following command:
```bash
    http-server .
```
This will start the server and display the server URL.

### Step 5: Open the Project

Open the displayed server http://127.0.0.1:8080 URL in your browser and navigate to the `src` folder to work with your project.

That's it! You should now be able to run the tax calculator web app with Rust and WebAssembly. 🎉
