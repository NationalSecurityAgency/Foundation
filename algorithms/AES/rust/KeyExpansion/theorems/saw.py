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

# fn subword(w: Word) -> Word
class subword_Contract(Contract):
    def specification(self):
        a = self.fresh_var(array_ty(4, u8), "a")
        
        self.execute_func(a)
        
        self.returns(cry_f("SubWord {a}"))

# pub fn key_expansion(k: usize, key: &[Word]) -> Vec<Block>
class key_expansion_Contract(Contract):
    def __init__(self, k : int):
        super().__init__()
        self.k = k
    def specification(self):
        k = cry_f("{self.k} : [64]");
        (key, key_p) = ref_to_fresh(self, array_ty(self.k // 32, array_ty(4, u8)), name="key", read_only=True)
        
        self.execute_func(k, slice_value(key_p))
        
        # The use of slices means the potentially unused part of the
        # 15-block array is all zeros and we need to account for them.
        key_from_cryptol = cry_f("KeyExpansion`{{{self.k}}} {key} # zero : [15][16][8]")

        self.returns(key_from_cryptol)

class KeyExpansion(unittest.TestCase):
    def test_KeyExpansion(self):
        connect(reset_server=True)
        if __name__ == "__main__": view(LogResults(verbose_failure=True))

        basedir = Path(__file__).absolute().parents[1] # Get absolute path to `Cargo.toml`
        crypath = basedir/"../../KeyExpansion.cry"
        mirpath = basedir/"linked-mir.json"

        cryptol_load_file(str(crypath))
        mod = mir_load_module(str(mirpath))

        subword_result = mir_verify(mod,
                                    'KeyExpansion::subword',
                                     subword_Contract(),
                                     script=ProofScript([rme]))
        self.assertIs(subword_result.is_success(), True)

        def key_expand_result(k):
            result = mir_verify(mod,
                               'KeyExpansion::key_expansion',
                               key_expansion_Contract(k),
                               lemmas=[subword_result],
                               script=ProofScript([z3(["SubWord"])]))
            self.assertIs(result.is_success(), True)
            return result
        
        key_expand_128_result = key_expand_result(128)
        key_expand_192_result = key_expand_result(192)
        key_expand_256_result = key_expand_result(256)
        
if __name__ == "__main__":
    unittest.main()
