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

# fn F(A: &mut State) {
class mut_state_Contract(Contract):
    def __init__(self, F : CryptolTerm):
        super().__init__()
        self.F = F
    def specification(self):
        (A, A_ref) = ref_to_fresh(self, array_ty(5, array_ty(5, u64)), "A")
        self.execute_func(A_ref)
        self.points_to(A_ref, cry_f("{self.F} {A}"))
        self.returns(void)

def mut_state_verify(test, mod, F_rust, F_cryptol):
    """ Verify a function of type `fn F(A: &mut State)`."""
    result = mir_verify(mod, F_rust, mut_state_Contract(cry_f(F_cryptol)),
                        script=ProofScript([rme]))
    test.assertIs(result.is_success(), True)
    return result

class Keccak_p_Contract(Contract):
    def specification(self):
        (A, A_ref) = ref_to_fresh(self, array_ty(5, array_ty(5, u64)), "A")
        self.execute_func(A_ref)
        self.points_to(A_ref, cry_f("toState (Keccak_p`{{w=64, nr=24}} (toString {A}))"))
        self.returns(void)

def Keccak_p_verify(test, mod, lemmas):
    result = mir_verify(mod, "Keccak::Keccak_p", Keccak_p_Contract(), lemmas,
                        script=ProofScript([z3("θ ρ π χ".split())]))
    test.assertIs(result.is_success(), True)
    return result

class Keccak(unittest.TestCase):
    def test_Cipher(self):
        connect(reset_server=True)
        if __name__ == "__main__": view(LogResults(verbose_failure=True))

        basedir = Path(__file__).absolute().parents[1] # Get absolute path to `Cargo.toml`
        StepMappings_crypath = basedir/"../StepMappings.cry"
        StateConversions_crypath = basedir/"../StateConversions.cry"
        mirpath = basedir/"linked-mir.json"

        cryptol_load_file(str(StepMappings_crypath))
        cryptol_load_file(str(StateConversions_crypath))
        mod = mir_load_module(str(mirpath))

        theta_result = mut_state_verify(self, mod, 'Keccak::theta', 'θ')
        rho_result = mut_state_verify(self, mod, 'Keccak::rho', 'ρ')
        pi_result = mut_state_verify(self, mod, 'Keccak::pi', 'π')
        chi_result = mut_state_verify(self, mod, 'Keccak::chi', 'χ')
        Keccak_p_result = Keccak_p_verify(self, mod, [theta_result, rho_result, pi_result, chi_result])

if __name__ == "__main__":
    unittest.main()
