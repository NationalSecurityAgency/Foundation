from pathlib import Path
import unittest
from saw_client             import *
from saw_client.crucible    import * 
from saw_client.mir         import * 
from saw_client.proofscript import *
from saw_client.mir_type    import *

def ref_to_fresh(c : Contract, ty : MIRType, name : Optional[str] = None,
                 read_only : bool = False) -> Tuple[FreshVar, SetupVal]:
    """Add to ``Contract`` ``c`` an allocation of a reference of type ``ty`` initialized to an unknown fresh value.
    If ``read_only == True`` then the allocated memory is immutable.

    :returns A fresh variable bound to the reference's initial value and the newly allocated reference. (The fresh
             variable will be assigned ``name`` if provided/available.)"""
    var = c.fresh_var(ty, name)
    ptr = c.alloc(ty, points_to = var, read_only = read_only)
    return (var, ptr)

# fn F(xs: Block) -> Block {
class block_to_block_Contract(Contract):
    def __init__(self, F : CryptolTerm):
        super().__init__()
        self.F = F
    def specification(self):
        xs = self.fresh_var(array_ty(16, u8), "xs")
        self.execute_func(xs)
        self.returns(cry_f("{self.F} {xs}"))

def block_to_block_verify(self, mod, F_rust, F_cryptol):
    result = mir_verify(mod, F_rust, block_to_block_Contract(cry_f(F_cryptol)),
                        script=ProofScript([rme]))
    self.assertIs(result.is_success(), True)
    return result

# fn F(xs: Block, ki: Block) -> Block {
# Matches `round` and `inv_round`
class round_Contract(Contract):
    def __init__(self, F : CryptolTerm):
        super().__init__()
        self.F = F
    def specification(self):
        xs = self.fresh_var(array_ty(16, u8), "xs")
        ki = self.fresh_var(array_ty(16, u8), "ki")
        self.execute_func(xs, ki)
        self.returns(cry_f("{self.F} {xs} {ki}"))

def round_verify(self, mod, F_rust, F_cryptol, lemmas, script=ProofScript([rme])):
    result = mir_verify(mod, F_rust, round_Contract(cry_f(F_cryptol)),
                        lemmas=lemmas,
                        script=script)
    self.assertIs(result.is_success(), True)
    return result

# fn F(ks: &[Block], plaintext: Block) -> Block {
# Matches `cipher` and `inv_cipher`
class cipher_Contract(Contract):
    def __init__(self, k: int, F: CryptolTerm):
        super().__init__()
        self.k = k
        self.F = F
    def specification(self):
        k = cry_f("{self.k} : [32]");
        (ks, ks_p) = ref_to_fresh(self, array_ty(self.k/32 + 6 + 1, array_ty(16, u8)), name="ks", read_only=True)
        plaintext = self.fresh_var(array_ty(16, u8), "plaintext")
        self.execute_func(slice_value(ks_p), plaintext)
        self.returns(cry_f("{self.F}`{{{self.k}}} {ks} {plaintext}"))

def cipher_verify(self, mod, k, F_rust, F_cryptol, lemmas, script=ProofScript([rme])):
    result = mir_verify(mod, F_rust, cipher_Contract(k, cry_f(F_cryptol)),
                        lemmas=lemmas,
                        script=script)
    self.assertIs(result.is_success(), True)
    return result

class Cipher(unittest.TestCase):
    def test_Cipher(self):
        connect(reset_server=True)
        if __name__ == "__main__": view(LogResults(verbose_failure=True))

        basedir = Path(__file__).absolute().parents[1] # Get absolute path to `Cargo.toml`
        crypath = basedir/"../../Cipher.cry"
        mirpath = basedir/"linked-mir.json"

        cryptol_load_file(str(crypath))
        mod = mir_load_module(str(mirpath))

        subbytes_result = block_to_block_verify(self, mod, 'Cipher::subbytes', 'SubBytes')
        inv_subbytes_result = block_to_block_verify(self, mod, 'Cipher::inv_subbytes', 'InvSubBytes')
        shiftrows_result = block_to_block_verify(self, mod, 'Cipher::shiftrows', 'ShiftRows')
        inv_shiftrows_result = block_to_block_verify(self, mod, 'Cipher::inv_shiftrows', 'InvShiftRows')
        mixcolums_result = block_to_block_verify(self, mod, 'Cipher::mixcolumns', 'MixColumns')
        inv_mixcolums_result = block_to_block_verify(self, mod, 'Cipher::inv_mixcolumns', 'InvMixColumns')

        round_result = round_verify(self, mod, 'Cipher::round', "Round", 
                                    lemmas=[subbytes_result, shiftrows_result, mixcolums_result])
        inv_round_result = round_verify(self, mod, 'Cipher::inv_round', "InvRound'",
                                        lemmas=[inv_subbytes_result, inv_shiftrows_result, inv_mixcolums_result],
                                        script=ProofScript([z3(["InvSubBytes", "InvShiftRows", "InvMixColumns"])]))

        def cipher_result(k):
            return cipher_verify(self, mod, k, 'Cipher::cipher', "Cipher",
                                 lemmas=[round_result, subbytes_result, shiftrows_result],
                                 script=ProofScript([z3(["Round", "SubBytes", "ShiftRows"])]))
        cipher_128_result = cipher_result(128)
        cipher_192_result = cipher_result(192)
        cipher_256_result = cipher_result(256)

        def inv_cipher_result(k):
            return cipher_verify(self, mod, k, 'Cipher::inv_cipher', "InvCipher'",
                                 lemmas=[inv_round_result, inv_subbytes_result, inv_shiftrows_result],
                                 script=ProofScript([cvc5(["InvRound", "InvSubBytes", "InvShiftRows"])]))
        inv_cipher_128_result = inv_cipher_result(128)
        inv_cipher_192_result = inv_cipher_result(192)
        inv_cipher_256_result = inv_cipher_result(256)

if __name__ == "__main__":
    unittest.main()
