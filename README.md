# Connect 4/TOOT and OTTO
## Prerequisites
- Rust toolchain (rustup, rustc, cargo)
- MongoDB
- [npm](https://www.npmjs.com/get-npm)
- Rust Nightly

## How to Use
1. Ensure MongoDB is installed and there is an existing database for the application to be able to connect to. MongoDB can be installed here. By default, the application makes calls to a database called ‘Connect4DB’. The application database can be configured in
the Rocket.toml file in the backend folder.
2. Ensure using the latest version of rust nightly by running: `rustup default nightly` on the terminal
3. Install `cargo-web` to run the client; do this by running `cargo install -f cargo-web` on the terminal
4. Provide the correct permissions to the run script by entering `chmod +x run.sh`
5. Run the source code by running the run script by entering `./run.sh`
6. If you run into an OpenSSL issue in the initial run of the project on a Linux machine (i.e. Ubuntu), run `sudo apt-get update -y` followed by `sudo apt-get install -y pkg-config`. If still unsuccessful, run `sudo apt-get install -y libssl-dev`
7. Navigate to `http://localhost:8000` on a web browser of your choosing in order to start playing!

### Notes
#### System Limitations
Our system limitations mostly include those that had already existed in the provided MEAN stack/Javascript project. One of these limitations include that the computerized opponent will always be the second player in both games. It will always play with yellow (“Y”) in Connect4, and “OTTO” (yellow) in TOOT and OTTO. Additionally, users cannot “undo” moves and game history is only ever saved once a game has been completed (i.e. either a user or computer has won a game). A limitation with the computerized opponent for TOOT and OTTO is that it does not actually play by the rules of TOOT and OTTO to spell its phrase (OTTO in our case) but rather plays by the Connect 4 logic of completing 4 tiles in a row. 

One limitation is that in order to play a new game after finishing one, a user should navigate back to the homepage (i.e. http://localhost:8000/) and then return to the appropriate page to play a new game. Another limitation is that pages cannot be refreshed. A user should again navigate to another page and return to the desired page to refresh it. Window resizing does not necessarily function as expected in certain web browsers, such as the navigation toolbar not moving/resizing dynamically. Another limitation we have is that sometimes the server connection to MongoDB is unstable and at times may not successfully post a finished game to
the connected database. 

Additionally, there are a presumed number of dependencies assumed to have been installed locally on a user’s computer to actually be able to set up and run the application. These dependencies may also only have other issues on non-Unix machines (i.e. Windows), though there does not seem to be issues with installing on MacOS or Linux (Ubuntu). 

Another limitation that is specific to our implementation is that a version of rust nightly must be used. This is because our backend uses the Rocket framework, which currently does not have a release on stable Rust (though it is expected a stable build will be released soon).
