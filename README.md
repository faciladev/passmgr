# passmgr
A command line password manager built with Rust programming language.
This is a minimalist and offline password manager that helps generate passwords for all applications you'd normally need to authenticate on using an email or username and password. 

This command line tool generates and saves a SHA512 hash text of your master password to a sqlite database located on the same directory. Another hashed text (SHA512) created using the answer you provide to a security question will be used as a salt when generating the master password hash.

Once the master password and the salt hashes are created and saved to the database you can login using the master password and then proceed to generate as many passwords as you want for every pair of email and application as needed. But only the master password hash and the salt hash are saved to the database for subsequent logins. Application specific passwords are generated taking the unhashed version of the master password, email/username and the name of the application as an input and utilizing the SHA256 hashing algorithm.
