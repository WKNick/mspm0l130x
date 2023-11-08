# [doc = "Register `ISET` writer"] pub type W = crate :: W < ISET_SPEC > ; # [doc = "Sets COMPIFG in RIS register\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum ISET_COMPIFG_AW { # [doc = "0: NO_EFFECT"] ISET_COMPIFG_NO_EFFECT = 0 , # [doc = "1: SET"] ISET_COMPIFG_SET = 1 , } impl From < ISET_COMPIFG_AW > for bool { # [inline (always)] fn from (variant : ISET_COMPIFG_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `ISET_COMPIFG` writer - Sets COMPIFG in RIS register"] pub type ISET_COMPIFG_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , ISET_COMPIFG_AW > ; impl < 'a , REG , const O : u8 > ISET_COMPIFG_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NO_EFFECT"] # [inline (always)] pub fn iset_compifg_no_effect (self) -> & 'a mut crate :: W < REG > { self . variant (ISET_COMPIFG_AW :: ISET_COMPIFG_NO_EFFECT) } # [doc = "SET"] # [inline (always)] pub fn iset_compifg_set (self) -> & 'a mut crate :: W < REG > { self . variant (ISET_COMPIFG_AW :: ISET_COMPIFG_SET) } } # [doc = "Sets COMPINVIFG in RIS register\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum ISET_COMPINVIFG_AW { # [doc = "0: NO_EFFECT"] ISET_COMPINVIFG_NO_EFFECT = 0 , # [doc = "1: SET"] ISET_COMPINVIFG_SET = 1 , } impl From < ISET_COMPINVIFG_AW > for bool { # [inline (always)] fn from (variant : ISET_COMPINVIFG_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `ISET_COMPINVIFG` writer - Sets COMPINVIFG in RIS register"] pub type ISET_COMPINVIFG_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , ISET_COMPINVIFG_AW > ; impl < 'a , REG , const O : u8 > ISET_COMPINVIFG_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NO_EFFECT"] # [inline (always)] pub fn iset_compinvifg_no_effect (self) -> & 'a mut crate :: W < REG > { self . variant (ISET_COMPINVIFG_AW :: ISET_COMPINVIFG_NO_EFFECT) } # [doc = "SET"] # [inline (always)] pub fn iset_compinvifg_set (self) -> & 'a mut crate :: W < REG > { self . variant (ISET_COMPINVIFG_AW :: ISET_COMPINVIFG_SET) } } # [doc = "Sets OUTRDYIFG in RIS register\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum ISET_OUTRDYIFG_AW { # [doc = "0: NO_EFFECT"] ISET_OUTRDYIFG_NO_EFFECT = 0 , # [doc = "1: SET"] ISET_OUTRDYIFG_SET = 1 , } impl From < ISET_OUTRDYIFG_AW > for bool { # [inline (always)] fn from (variant : ISET_OUTRDYIFG_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `ISET_OUTRDYIFG` writer - Sets OUTRDYIFG in RIS register"] pub type ISET_OUTRDYIFG_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , ISET_OUTRDYIFG_AW > ; impl < 'a , REG , const O : u8 > ISET_OUTRDYIFG_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NO_EFFECT"] # [inline (always)] pub fn iset_outrdyifg_no_effect (self) -> & 'a mut crate :: W < REG > { self . variant (ISET_OUTRDYIFG_AW :: ISET_OUTRDYIFG_NO_EFFECT) } # [doc = "SET"] # [inline (always)] pub fn iset_outrdyifg_set (self) -> & 'a mut crate :: W < REG > { self . variant (ISET_OUTRDYIFG_AW :: ISET_OUTRDYIFG_SET) } } impl W { # [doc = "Bit 1 - Sets COMPIFG in RIS register"] # [inline (always)] # [must_use] pub fn iset_compifg (& mut self) -> ISET_COMPIFG_W < ISET_SPEC , 1 > { ISET_COMPIFG_W :: new (self) } # [doc = "Bit 2 - Sets COMPINVIFG in RIS register"] # [inline (always)] # [must_use] pub fn iset_compinvifg (& mut self) -> ISET_COMPINVIFG_W < ISET_SPEC , 2 > { ISET_COMPINVIFG_W :: new (self) } # [doc = "Bit 3 - Sets OUTRDYIFG in RIS register"] # [inline (always)] # [must_use] pub fn iset_outrdyifg (& mut self) -> ISET_OUTRDYIFG_W < ISET_SPEC , 3 > { ISET_OUTRDYIFG_W :: new (self) } # [doc = r" Writes raw bits to the register."] # [doc = r""] # [doc = r" # Safety"] # [doc = r""] # [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "Interrupt set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct ISET_SPEC ; impl crate :: RegisterSpec for ISET_SPEC { type Ux = u32 ; } # [doc = "`write(|w| ..)` method takes [`iset::W`](W) writer structure"] impl crate :: Writable for ISET_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets ISET to value 0"] impl crate :: Resettable for ISET_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }