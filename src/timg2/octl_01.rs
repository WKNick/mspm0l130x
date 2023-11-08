# [doc = "Register `OCTL_01[%s]` reader"] pub type R = crate :: R < OCTL_01_SPEC > ; # [doc = "Register `OCTL_01[%s]` writer"] pub type W = crate :: W < OCTL_01_SPEC > ; # [doc = "Field `OCTL_01_CCPO` reader - CCP Output Source"] pub type OCTL_01_CCPO_R = crate :: FieldReader < OCTL_01_CCPO_A > ; # [doc = "CCP Output Source\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum OCTL_01_CCPO_A { # [doc = "0: FUNCVAL"] OCTL_01_CCPO_FUNCVAL = 0 , # [doc = "1: LOAD"] OCTL_01_CCPO_LOAD = 1 , # [doc = "2: CMPVAL"] OCTL_01_CCPO_CMPVAL = 2 , # [doc = "4: ZERO"] OCTL_01_CCPO_ZERO = 4 , # [doc = "5: CAPCOND"] OCTL_01_CCPO_CAPCOND = 5 , # [doc = "6: FAULTCOND"] OCTL_01_CCPO_FAULTCOND = 6 , # [doc = "8: CC0_MIRROR_ALL"] OCTL_01_CCPO_CC0_MIRROR_ALL = 8 , # [doc = "9: CC1_MIRROR_ALL"] OCTL_01_CCPO_CC1_MIRROR_ALL = 9 , # [doc = "12: DEADBAND"] OCTL_01_CCPO_DEADBAND = 12 , # [doc = "13: CNTDIR"] OCTL_01_CCPO_CNTDIR = 13 , } impl From < OCTL_01_CCPO_A > for u8 { # [inline (always)] fn from (variant : OCTL_01_CCPO_A) -> Self { variant as _ } } impl crate :: FieldSpec for OCTL_01_CCPO_A { type Ux = u8 ; } impl OCTL_01_CCPO_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Option < OCTL_01_CCPO_A > { match self . bits { 0 => Some (OCTL_01_CCPO_A :: OCTL_01_CCPO_FUNCVAL) , 1 => Some (OCTL_01_CCPO_A :: OCTL_01_CCPO_LOAD) , 2 => Some (OCTL_01_CCPO_A :: OCTL_01_CCPO_CMPVAL) , 4 => Some (OCTL_01_CCPO_A :: OCTL_01_CCPO_ZERO) , 5 => Some (OCTL_01_CCPO_A :: OCTL_01_CCPO_CAPCOND) , 6 => Some (OCTL_01_CCPO_A :: OCTL_01_CCPO_FAULTCOND) , 8 => Some (OCTL_01_CCPO_A :: OCTL_01_CCPO_CC0_MIRROR_ALL) , 9 => Some (OCTL_01_CCPO_A :: OCTL_01_CCPO_CC1_MIRROR_ALL) , 12 => Some (OCTL_01_CCPO_A :: OCTL_01_CCPO_DEADBAND) , 13 => Some (OCTL_01_CCPO_A :: OCTL_01_CCPO_CNTDIR) , _ => None , } } # [doc = "FUNCVAL"] # [inline (always)] pub fn is_octl_01_ccpo_funcval (& self) -> bool { * self == OCTL_01_CCPO_A :: OCTL_01_CCPO_FUNCVAL } # [doc = "LOAD"] # [inline (always)] pub fn is_octl_01_ccpo_load (& self) -> bool { * self == OCTL_01_CCPO_A :: OCTL_01_CCPO_LOAD } # [doc = "CMPVAL"] # [inline (always)] pub fn is_octl_01_ccpo_cmpval (& self) -> bool { * self == OCTL_01_CCPO_A :: OCTL_01_CCPO_CMPVAL } # [doc = "ZERO"] # [inline (always)] pub fn is_octl_01_ccpo_zero (& self) -> bool { * self == OCTL_01_CCPO_A :: OCTL_01_CCPO_ZERO } # [doc = "CAPCOND"] # [inline (always)] pub fn is_octl_01_ccpo_capcond (& self) -> bool { * self == OCTL_01_CCPO_A :: OCTL_01_CCPO_CAPCOND } # [doc = "FAULTCOND"] # [inline (always)] pub fn is_octl_01_ccpo_faultcond (& self) -> bool { * self == OCTL_01_CCPO_A :: OCTL_01_CCPO_FAULTCOND } # [doc = "CC0_MIRROR_ALL"] # [inline (always)] pub fn is_octl_01_ccpo_cc0_mirror_all (& self) -> bool { * self == OCTL_01_CCPO_A :: OCTL_01_CCPO_CC0_MIRROR_ALL } # [doc = "CC1_MIRROR_ALL"] # [inline (always)] pub fn is_octl_01_ccpo_cc1_mirror_all (& self) -> bool { * self == OCTL_01_CCPO_A :: OCTL_01_CCPO_CC1_MIRROR_ALL } # [doc = "DEADBAND"] # [inline (always)] pub fn is_octl_01_ccpo_deadband (& self) -> bool { * self == OCTL_01_CCPO_A :: OCTL_01_CCPO_DEADBAND } # [doc = "CNTDIR"] # [inline (always)] pub fn is_octl_01_ccpo_cntdir (& self) -> bool { * self == OCTL_01_CCPO_A :: OCTL_01_CCPO_CNTDIR } } # [doc = "Field `OCTL_01_CCPO` writer - CCP Output Source"] pub type OCTL_01_CCPO_W < 'a , REG , const O : u8 > = crate :: FieldWriter < 'a , REG , 4 , O , OCTL_01_CCPO_A > ; impl < 'a , REG , const O : u8 > OCTL_01_CCPO_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , REG :: Ux : From < u8 > { # [doc = "FUNCVAL"] # [inline (always)] pub fn octl_01_ccpo_funcval (self) -> & 'a mut crate :: W < REG > { self . variant (OCTL_01_CCPO_A :: OCTL_01_CCPO_FUNCVAL) } # [doc = "LOAD"] # [inline (always)] pub fn octl_01_ccpo_load (self) -> & 'a mut crate :: W < REG > { self . variant (OCTL_01_CCPO_A :: OCTL_01_CCPO_LOAD) } # [doc = "CMPVAL"] # [inline (always)] pub fn octl_01_ccpo_cmpval (self) -> & 'a mut crate :: W < REG > { self . variant (OCTL_01_CCPO_A :: OCTL_01_CCPO_CMPVAL) } # [doc = "ZERO"] # [inline (always)] pub fn octl_01_ccpo_zero (self) -> & 'a mut crate :: W < REG > { self . variant (OCTL_01_CCPO_A :: OCTL_01_CCPO_ZERO) } # [doc = "CAPCOND"] # [inline (always)] pub fn octl_01_ccpo_capcond (self) -> & 'a mut crate :: W < REG > { self . variant (OCTL_01_CCPO_A :: OCTL_01_CCPO_CAPCOND) } # [doc = "FAULTCOND"] # [inline (always)] pub fn octl_01_ccpo_faultcond (self) -> & 'a mut crate :: W < REG > { self . variant (OCTL_01_CCPO_A :: OCTL_01_CCPO_FAULTCOND) } # [doc = "CC0_MIRROR_ALL"] # [inline (always)] pub fn octl_01_ccpo_cc0_mirror_all (self) -> & 'a mut crate :: W < REG > { self . variant (OCTL_01_CCPO_A :: OCTL_01_CCPO_CC0_MIRROR_ALL) } # [doc = "CC1_MIRROR_ALL"] # [inline (always)] pub fn octl_01_ccpo_cc1_mirror_all (self) -> & 'a mut crate :: W < REG > { self . variant (OCTL_01_CCPO_A :: OCTL_01_CCPO_CC1_MIRROR_ALL) } # [doc = "DEADBAND"] # [inline (always)] pub fn octl_01_ccpo_deadband (self) -> & 'a mut crate :: W < REG > { self . variant (OCTL_01_CCPO_A :: OCTL_01_CCPO_DEADBAND) } # [doc = "CNTDIR"] # [inline (always)] pub fn octl_01_ccpo_cntdir (self) -> & 'a mut crate :: W < REG > { self . variant (OCTL_01_CCPO_A :: OCTL_01_CCPO_CNTDIR) } } # [doc = "Field `OCTL_01_CCPOINV` reader - CCP Output Invert The output as selected by CCPO is conditionally inverted."] pub type OCTL_01_CCPOINV_R = crate :: BitReader < OCTL_01_CCPOINV_A > ; # [doc = "CCP Output Invert The output as selected by CCPO is conditionally inverted.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum OCTL_01_CCPOINV_A { # [doc = "0: NOINV"] OCTL_01_CCPOINV_NOINV = 0 , # [doc = "1: INV"] OCTL_01_CCPOINV_INV = 1 , } impl From < OCTL_01_CCPOINV_A > for bool { # [inline (always)] fn from (variant : OCTL_01_CCPOINV_A) -> Self { variant as u8 != 0 } } impl OCTL_01_CCPOINV_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> OCTL_01_CCPOINV_A { match self . bits { false => OCTL_01_CCPOINV_A :: OCTL_01_CCPOINV_NOINV , true => OCTL_01_CCPOINV_A :: OCTL_01_CCPOINV_INV , } } # [doc = "NOINV"] # [inline (always)] pub fn is_octl_01_ccpoinv_noinv (& self) -> bool { * self == OCTL_01_CCPOINV_A :: OCTL_01_CCPOINV_NOINV } # [doc = "INV"] # [inline (always)] pub fn is_octl_01_ccpoinv_inv (& self) -> bool { * self == OCTL_01_CCPOINV_A :: OCTL_01_CCPOINV_INV } } # [doc = "Field `OCTL_01_CCPOINV` writer - CCP Output Invert The output as selected by CCPO is conditionally inverted."] pub type OCTL_01_CCPOINV_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , OCTL_01_CCPOINV_A > ; impl < 'a , REG , const O : u8 > OCTL_01_CCPOINV_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NOINV"] # [inline (always)] pub fn octl_01_ccpoinv_noinv (self) -> & 'a mut crate :: W < REG > { self . variant (OCTL_01_CCPOINV_A :: OCTL_01_CCPOINV_NOINV) } # [doc = "INV"] # [inline (always)] pub fn octl_01_ccpoinv_inv (self) -> & 'a mut crate :: W < REG > { self . variant (OCTL_01_CCPOINV_A :: OCTL_01_CCPOINV_INV) } } # [doc = "Field `OCTL_01_CCPIV` reader - CCP Initial Value This bit specifies the logical value put on the signal generator state while the counter is disabled (CTRCTL.EN == 0)."] pub type OCTL_01_CCPIV_R = crate :: BitReader < OCTL_01_CCPIV_A > ; # [doc = "CCP Initial Value This bit specifies the logical value put on the signal generator state while the counter is disabled (CTRCTL.EN == 0).\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum OCTL_01_CCPIV_A { # [doc = "0: LOW"] OCTL_01_CCPIV_LOW = 0 , # [doc = "1: HIGH"] OCTL_01_CCPIV_HIGH = 1 , } impl From < OCTL_01_CCPIV_A > for bool { # [inline (always)] fn from (variant : OCTL_01_CCPIV_A) -> Self { variant as u8 != 0 } } impl OCTL_01_CCPIV_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> OCTL_01_CCPIV_A { match self . bits { false => OCTL_01_CCPIV_A :: OCTL_01_CCPIV_LOW , true => OCTL_01_CCPIV_A :: OCTL_01_CCPIV_HIGH , } } # [doc = "LOW"] # [inline (always)] pub fn is_octl_01_ccpiv_low (& self) -> bool { * self == OCTL_01_CCPIV_A :: OCTL_01_CCPIV_LOW } # [doc = "HIGH"] # [inline (always)] pub fn is_octl_01_ccpiv_high (& self) -> bool { * self == OCTL_01_CCPIV_A :: OCTL_01_CCPIV_HIGH } } # [doc = "Field `OCTL_01_CCPIV` writer - CCP Initial Value This bit specifies the logical value put on the signal generator state while the counter is disabled (CTRCTL.EN == 0)."] pub type OCTL_01_CCPIV_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , OCTL_01_CCPIV_A > ; impl < 'a , REG , const O : u8 > OCTL_01_CCPIV_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "LOW"] # [inline (always)] pub fn octl_01_ccpiv_low (self) -> & 'a mut crate :: W < REG > { self . variant (OCTL_01_CCPIV_A :: OCTL_01_CCPIV_LOW) } # [doc = "HIGH"] # [inline (always)] pub fn octl_01_ccpiv_high (self) -> & 'a mut crate :: W < REG > { self . variant (OCTL_01_CCPIV_A :: OCTL_01_CCPIV_HIGH) } } impl R { # [doc = "Bits 0:3 - CCP Output Source"] # [inline (always)] pub fn octl_01_ccpo (& self) -> OCTL_01_CCPO_R { OCTL_01_CCPO_R :: new ((self . bits & 0x0f) as u8) } # [doc = "Bit 4 - CCP Output Invert The output as selected by CCPO is conditionally inverted."] # [inline (always)] pub fn octl_01_ccpoinv (& self) -> OCTL_01_CCPOINV_R { OCTL_01_CCPOINV_R :: new (((self . bits >> 4) & 1) != 0) } # [doc = "Bit 5 - CCP Initial Value This bit specifies the logical value put on the signal generator state while the counter is disabled (CTRCTL.EN == 0)."] # [inline (always)] pub fn octl_01_ccpiv (& self) -> OCTL_01_CCPIV_R { OCTL_01_CCPIV_R :: new (((self . bits >> 5) & 1) != 0) } } impl W { # [doc = "Bits 0:3 - CCP Output Source"] # [inline (always)] # [must_use] pub fn octl_01_ccpo (& mut self) -> OCTL_01_CCPO_W < OCTL_01_SPEC , 0 > { OCTL_01_CCPO_W :: new (self) } # [doc = "Bit 4 - CCP Output Invert The output as selected by CCPO is conditionally inverted."] # [inline (always)] # [must_use] pub fn octl_01_ccpoinv (& mut self) -> OCTL_01_CCPOINV_W < OCTL_01_SPEC , 4 > { OCTL_01_CCPOINV_W :: new (self) } # [doc = "Bit 5 - CCP Initial Value This bit specifies the logical value put on the signal generator state while the counter is disabled (CTRCTL.EN == 0)."] # [inline (always)] # [must_use] pub fn octl_01_ccpiv (& mut self) -> OCTL_01_CCPIV_W < OCTL_01_SPEC , 5 > { OCTL_01_CCPIV_W :: new (self) } # [doc = r" Writes raw bits to the register."] # [doc = r""] # [doc = r" # Safety"] # [doc = r""] # [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "CCP Output Control Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`octl_01::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`octl_01::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct OCTL_01_SPEC ; impl crate :: RegisterSpec for OCTL_01_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`octl_01::R`](R) reader structure"] impl crate :: Readable for OCTL_01_SPEC { } # [doc = "`write(|w| ..)` method takes [`octl_01::W`](W) writer structure"] impl crate :: Writable for OCTL_01_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets OCTL_01[%s]
to value 0"] impl crate :: Resettable for OCTL_01_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }