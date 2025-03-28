# insanely-floaty
To whomever reads this code, I am sorry

This is a reversing CTF challenge based on [How to Securely Implement Cryptography in Deep Neural Networks](https://eprint.iacr.org/2025/288.pdf) a 2025 paper by David Gerault, Anna Hambitzer, Eyal Ronen, and Adi Shamir.

I had the idea for the challenge after seeing Adi Shamir present the work at [RWC 2025](https://rwc.iacr.org/2025/).

The challenge represents the unsanitised natural implementation of AES with parameter c = 1.0.
This leads to a linear time key recovery attack by jiggling the floats around a tiny amount and observing whether the output of the encryption operation changes.

Time dependent I would also like to implement a part 2 challenge based on the naïve sanitisation of inputs limiting them from 0.0 <= x <= 1.0, this is still vulnerable but leads to a more complex attack where you do some differential cryptanalysis of the interaction of the floats with the sbox.
