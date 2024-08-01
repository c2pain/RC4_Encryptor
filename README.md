# RC4_Encryptor
A quick-build Encryptor for C# binaries and raw shellcode using RC4 encryption, combined with a "Loader" for enhanced obfuscation. By default, it uses RC4 encryption to encrypt C# binaries with the key "C2Pain".

## Usage
You can simply input the CSharp binary as argument:
```
C:\Users\C2Pain\Desktop> rc4_encryptor.exe
[!] Usage: rc4_encryptor.exe <input_filename_to_encrypt>
[!] Example: rc4_encryptor.exe Seatbelt.exe


C:\Users\C2Pain\Desktop> rc4_encryptor.exe Seatbelt.exe
[+] Encrypted shellcode saved to: S-e-a-t-b-e-l-t-4.enc
```
