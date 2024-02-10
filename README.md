# Personal Website - Contact Page

Link to Personal Website Contact Page: [Contact Me](https://zolawebsite-mjh140-84221a449a6f2f4baac19b4d09c0cf8992f2fddab7c3.gitlab.io/contact/)

---

## Summary

This project builds on my personal website and adds customized functionality to the form submission process on my *Contact Me* page. When the website was first built, the messages submitted through my website were processed using the free form processer FormSpree. FormSpree is exceptionally easy to integrate into an html form, but the data processing structure is predefined and inflexible to integrating with other services. In an attempt to learn more about processing data through web applications, I decided to replace the FormSpree functionality with a custom pipeline built with AWS lambda and DynamoDB. With the new data pipeline, the form data is sent as an http POST request to an AWS lambda function, where the JSON data is deserialized and uploaded to a DynamoDB database. The client side view along with the backend database update is shown in the video below.

[Video Demonstration](/workspaces/ContactPage_lambda/ContactPage.mp4)

## How It Works

The front end of the form did not change from the previous iteration. The form fields remained the same (Name, Email, and Message) and the http request stayed the same as well `method='POST'`. In order to package the JSON payload to be compatible with AWS API Gateway, a javascript `Event Handler` was wrapped around the bootstrap submit button to catch the data and reformat it before passing the http request. The JS `Fetch API` was configured with the json payload and the lambda function API Endpoint. 

Because my website is hosted on a different domain than the lambda function API (Gitlab vs AWS), the browser implements a security measure called `Cross Origin Resource Sharing (CORS)` that intercepts the request before allowing it to pass through to the API endpoint. CORS is a browser security measure intended to protect users from potential malicious websites trying to access user data from other domains/servers without permission. CORS policy, by default, blocks requests across domains and must be explicetely handled within the API in order to allow cross origin traffic. For this API, my personal website origin was added as a permitted CORS website, and the request is allowed to pass through.

The form data is now serialized as a JSON payload and has reached the lambda function API endpoint. The lambda function is broken into three asynchronous functions:

- **main**:
    - Instantiate AWS CloudWatch logging and DynamoDB client
    - Pass request to function_handler

- **function_handler**:
    - Handle http OPTIONS request (CORS)
    - Deserialize JSON event body into predefined Item struct
    - Pass item to add_item function
    - Return Status 200 if add_item is successful (else 400 with error handling)

- **add_item**:
    - Deconstruct item into DynamoDB attibutes
    - Generate partition key as concatination of email and timestamp
    - Insert data into database

If the data insertion is successful, DynamoDB will send a response confirming the process is complete. From the client side, a small bootstrap window appears that verifies the message was sent. 


## Why Rust

Rust is an efficient language for handling form data. Rust is able to create complex datastructures in the form of *structs* that when paired with pattern matching and the `serde` package, makes for efficient deserialization of JSON data. Rust's default variable immutability also ensures an added level of data security for handling user data.

Rust also has series of cargo subcommands called `cargo-lambda` that aids in deploying AWS Lambda functions written in Rust. The main advantage of using `cargo-lambda` is that it abstracts away many of the manual steps and complexities involved in cross compilation to ensure the local function is packaged into a binary compatible with AWS Lambda environment. Using the `cargo-lambda` subcommands with the `AWS CLI` makes for very efficient development and deployment of AWS lambda functions.

## Challenges

#### Cross Origin Resource Sharing (CORS)

This was my first introduction to CORS and it was not easy to diagnose the problem. Enabling CORS is not as simple as a checkbox (Like AWS would lead you to believe). Understanding how to allow cross origin resource sharing for your API requires an understanding of what the browser is doing when it intercepts your request. In attempt to clarify my own understanding I'll try to explain it here, but feel free to move ahead.

As stated earlier, the browser will intercept your http request and abide by the CORS security policy before allowing your request to pass to the API. The browser will then attempt to understand the APIs CORS policy and determine if this request is acceptable to pass through. If the request is a standard POST, GET, or HEAD method, then the browser will pass the request method to the API and waits for a response that tells the browser what headers within the request should be allowed through.

However... if the request is abnormal in any way - perhaps a DELETE request, CONNECT request, or a POST request compiled in a JS Event handler with custom headers - the browser won't send a request with the original method, it will send an *OPTIONS* request to your API and wait for the response. The response has to clarify which origins, methods, and headers are accepted for CORS. If the OPTIONS method is not explicitely handled within the API, the browser will default to blocking the request. My handling of the OPTIONS request can be seen in the `main.rs` file, `function_handler` function.

#### Choose your compiler wisely

```
  cargo:rerun-if-env-changed=RING_PREGENERATE_ASM
  cargo:rustc-env=RING_CORE_PREFIX=ring_core_0_17_7_
  OPT_LEVEL = Some("3")
  TARGET = Some("aarch64-unknown-linux-gnu")
  HOST = Some("x86_64-pc-windows-msvc")

  error: failed to run custom build command for `ring v0.17.7`
```

#### Spaces and Dashes in File Path

If you get this error when building, you have spaces or dashes in your file path.

```
  = note: error: Unknown Clang option: '-\'
```

#### Windows Cannot Build Locally - Argument List Too Long

```text
failed: exit code: 1
  = note: error(link): unable to spawn C:\Users\matth\scoop\apps\zig\current\zig.exe: NameTooLong
          error: UnableToSpawnSelf
```

[GitHub Issue #10881: zig cc can fail with extremely long argument list](https://github.com/ziglang/zig/issues/10881)

The creator of cargo lambda, [calavera](https://github.com/ziglang/zig/issues/10881#issuecomment-1100571469), acknowledged this issue and there doesn't appear to be a solution for Windows.

I moved my repository in to Github Codespaces and was able to build and deploy from Codespaces.


#### Future Work



