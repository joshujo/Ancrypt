# Ancrypt

## Table of Contents

- [What is Ancrypt?](#what-is-ancrypt)
- [Features](#features)
- [Getting Started](#getting-started)
- [Security Features](#security-features)
- [Security Limitations](#security-limitations)
- [Usage Recommendation](#usage-recommendation)
- [FAQ](#faq)
    - [What if I lose or forget my password?](#what-if-i-lose-or-forget-my-password)
    - [Can I sync my vaults across devices?](#can-i-sync-my-vaults-across-devices)
- [Disclaimer](#disclaimer)

## What is Ancrypt?

Ancrypt is a password manager that is fully offline and is simple to use. It is made purely for educational purposes and as a result, may not be suitable for real usage in high risk environments.

## What is currently included?

Ancrypt currently includes all the basic features of any password manager. It allows for multiple *"vaults"* to be created by the user with one master password that decrypts the vault. Passwords can easily be added to the vault by assigning it a name and an associated password, where it will be saved after encryption. Additionally, deleting passwords and vaults requires a 6-digit confirmation code to verify intention. This should prevent *most* forms of misinput. 

## Getting Started

1. Install the binary or an installer from [here](https://github.com/joshujo/Ancrypt/releases).

2. Open Ancrypt and press *"New Vault"* to open up a modal where you can create a new vault with a name and master password.

3. Click "Open Vault" and enter your password in to access the vault.

4. Create a new password to store by entering the name of the password and the password itself

5. Retrieve the password by clicking *"Copy to Clipboard"*, the moment you press this, the application will automatically clear your clipboard after 30 seconds.

6. Paste your password in the application that you need to paste it in, then press *"Clear Clipboard"* to ensure a lower exposure time.

7. Press *"Lock Vault"* when you are finished when the password manager. Ancrypt will automatically clear your clipboard when you lock the vault.

8. To delete any stored passwords or vaults, click the *bin* icon and enter the confirmation code to permanently discard the requested data.

9. Vaults are stored in `C:/users/{user}/Appdata/Roaming/Ancrypt/Vaults`

## What security features are included?

As of right now, the main security features are :

- Clipboard clears automatically after 30 seconds to minimise risk of other apps or malware reading the clipboard

- User can manually clear the clipboard immediately after use to further reduce the risk of a clipboard attack

- Files are encrypted in storage and can only be opened using a key derived from the user password. 

- The encryption algorithm used is PBKDF2 which has been audited and proven to be a reliable and secure algorithm. (Don't ask why I'm not using Argon2)

- There is a concise separation of frontend and backend, your frontend will never have access to any passwords except when you insert your passwords into Ancrypt, not even via IPC. This minimises the attack surface by ensuring that attackers have to either attack the Rust backend itself, or burrow into your system memory. 

- Ancrypt runs purely offline and locally on your device, ensuring that your passwords won't be intercepted over the internet.

- Each individual vault is uniquely salted ensuring a higher degree of security

## What security features **aren't included?**

- The memory isn't zeroed upon dropping the vault, it is merely freed and needs to be overwritten for the passwords to not be present in memory. This runs the risk of malware being able to perform a forensic analysis of system memory to extract dropped plaintext passwords freed in your memory.

- There may be additional security limitations not listed here, but to my knowledge, the above is the primary issue with this application that makes it insecure

## Should I use Ancrypt for more than just demonstration purposes?

I can't exactly recommend this as a primary password manager but, it should be decently safe and suitable in low risk environments, such as on a shared computer where you are sure that other users are not going to forensically sample your memory, or your computer is not infected with malware. Ancrypt is suitable for users who already store passwords on their device as plaintext as it adds another layer of security over what previous already had no security. Other than that and for demonstration purposes, **I really can not recommend the serious use of Ancrypt.**

## FAQ 

#### What if I lose or forget my password?

There is no way to recover the vault due to the security measures put in place.

#### Can I sync my vaults across devices?

Yes! Simply copy your vaults over from `C:/users/{user}/Appdata/Roaming/Ancrypt/Vaults` to your new device. Each file is self encrypted and is therefore portable.

## Disclaimer

***This software is provided "as-is", without warranty of any kind, and the author is not responsible for any damages arising from its use.***

