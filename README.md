# 🎅 Secret Santa

## Build + Run

Edit [src/emails.json](src/emails.json) and add all the people participating.

Sign up for [SendGrid](https://sendgrid.com/) and generate an API key. Place the API key in [src/sendgrid_api_key.txt](src/sendgrid_api_key.txt)

You may want to change the sender email in [src/main.rs](src/main.rs) to be something other than `sender@example.com`

When everything is set, you can send out the emails with:

```
cargo run
```

assuming you have [Rust](https://rustup.rs/) installed.

You can also set a seed on the random number generator so you can resend them later. _Technically_ you can use the seed to see who everyone got, but that's no fun.
