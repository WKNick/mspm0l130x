# [doc = "Register `IMASK` reader"] pub type R = crate :: R < IMASK_SPEC > ; # [doc = "Register `IMASK` writer"] pub type W = crate :: W < IMASK_SPEC > ; # [doc = "Field `IMASK_COMPIFG` reader - Masks COMPIFG"] pub type IMASK_COMPIFG_R = crate :: BitReader < IMASK_COMPIFG_A > ; # [doc = "Masks COMPIFG\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum IMASK_COMPIFG_A { # [doc = "0: CLR"] IMASK_COMPIFG_CLR = 0 , # [doc = "1: SET"] IMASK_COMPIFG_SET = 1 , } impl From < IMASK_COMPIFG_A > for bool { # [inline (always)] fn from (variant : IMASK_COMPIFG_A) -> Self { variant as u8 != 0 } } impl IMASK_COMPIFG_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> IMASK_COMPIFG_A { match self . bits { false => IMASK_COMPIFG_A :: IMASK_COMPIFG_CLR , true => IMASK_COMPIFG_A :: IMASK_COMPIFG_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_imask_compifg_clr (& self) -> bool { * self == IMASK_COMPIFG_A :: IMASK_COMPIFG_CLR } # [doc = "SET"] # [inline (always)] pub fn is_imask_compifg_set (& self) -> bool { * self == IMASK_COMPIFG_A :: IMASK_COMPIFG_SET } } # [doc = "Field `IMASK_COMPIFG` writer - Masks COMPIFG"] pub type IMASK_COMPIFG_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , IMASK_COMPIFG_A > ; impl < 'a , REG , const O : u8 > IMASK_COMPIFG_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn imask_compifg_clr (self) -> & 'a mut crate :: W < REG > { self . variant (IMASK_COMPIFG_A :: IMASK_COMPIFG_CLR) } # [doc = "SET"] # [inline (always)] pub fn imask_compifg_set (self) -> & 'a mut crate :: W < REG > { self . variant (IMASK_COMPIFG_A :: IMASK_COMPIFG_SET) } } # [doc = "Field `IMASK_COMPINVIFG` reader - Masks COMPINVIFG"] pub type IMASK_COMPINVIFG_R = crate :: BitReader < IMASK_COMPINVIFG_A > ; # [doc = "Masks COMPINVIFG\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum IMASK_COMPINVIFG_A { # [doc = "0: CLR"] IMASK_COMPINVIFG_CLR = 0 , # [doc = "1: SET"] IMASK_COMPINVIFG_SET = 1 , } impl From < IMASK_COMPINVIFG_A > for bool { # [inline (always)] fn from (variant : IMASK_COMPINVIFG_A) -> Self { variant as u8 != 0 } } impl IMASK_COMPINVIFG_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> IMASK_COMPINVIFG_A { match self . bits { false => IMASK_COMPINVIFG_A :: IMASK_COMPINVIFG_CLR , true => IMASK_COMPINVIFG_A :: IMASK_COMPINVIFG_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_imask_compinvifg_clr (& self) -> bool { * self == IMASK_COMPINVIFG_A :: IMASK_COMPINVIFG_CLR } # [doc = "SET"] # [inline (always)] pub fn is_imask_compinvifg_set (& self) -> bool { * self == IMASK_COMPINVIFG_A :: IMASK_COMPINVIFG_SET } } # [doc = "Field `IMASK_COMPINVIFG` writer - Masks COMPINVIFG"] pub type IMASK_COMPINVIFG_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , IMASK_COMPINVIFG_A > ; impl < 'a , REG , const O : u8 > IMASK_COMPINVIFG_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn imask_compinvifg_clr (self) -> & 'a mut crate :: W < REG > { self . variant (IMASK_COMPINVIFG_A :: IMASK_COMPINVIFG_CLR) } # [doc = "SET"] # [inline (always)] pub fn imask_compinvifg_set (self) -> & 'a mut crate :: W < REG > { self . variant (IMASK_COMPINVIFG_A :: IMASK_COMPINVIFG_SET) } } # [doc = "Field `IMASK_OUTRDYIFG` reader - Masks OUTRDYIFG"] pub type IMASK_OUTRDYIFG_R = crate :: BitReader < IMASK_OUTRDYIFG_A > ; # [doc = "Masks OUTRDYIFG\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum IMASK_OUTRDYIFG_A { # [doc = "0: CLR"] IMASK_OUTRDYIFG_CLR = 0 , # [doc = "1: SET"] IMASK_OUTRDYIFG_SET = 1 , } impl From < IMASK_OUTRDYIFG_A > for bool { # [inline (always)] fn from (variant : IMASK_OUTRDYIFG_A) -> Self { variant as u8 != 0 } } impl IMASK_OUTRDYIFG_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> IMASK_OUTRDYIFG_A { match self . bits { false => IMASK_OUTRDYIFG_A :: IMASK_OUTRDYIFG_CLR , true => IMASK_OUTRDYIFG_A :: IMASK_OUTRDYIFG_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_imask_outrdyifg_clr (& self) -> bool { * self == IMASK_OUTRDYIFG_A :: IMASK_OUTRDYIFG_CLR } # [doc = "SET"] # [inline (always)] pub fn is_imask_outrdyifg_set (& self) -> bool { * self == IMASK_OUTRDYIFG_A :: IMASK_OUTRDYIFG_SET } } # [doc = "Field `IMASK_OUTRDYIFG` writer - Masks OUTRDYIFG"] pub type IMASK_OUTRDYIFG_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , IMASK_OUTRDYIFG_A > ; impl < 'a , REG , const O : u8 > IMASK_OUTRDYIFG_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn imask_outrdyifg_clr (self) -> & 'a mut crate :: W < REG > { self . variant (IMASK_OUTRDYIFG_A :: IMASK_OUTRDYIFG_CLR) } # [doc = "SET"] # [inline (always)] pub fn imask_outrdyifg_set (self) -> & 'a mut crate :: W < REG > { self . variant (IMASK_OUTRDYIFG_A :: IMASK_OUTRDYIFG_SET) } } impl R { # [doc = "Bit 1 - Masks COMPIFG"] # [inline (always)] pub fn imask_compifg (& self) -> IMASK_COMPIFG_R { IMASK_COMPIFG_R :: new (((self . bits >> 1) & 1) != 0) } # [doc = "Bit 2 - Masks COMPINVIFG"] # [inline (always)] pub fn imask_compinvifg (& self) -> IMASK_COMPINVIFG_R { IMASK_COMPINVIFG_R :: new (((self . bits >> 2) & 1) != 0) } # [doc = "Bit 3 - Masks OUTRDYIFG"] # [inline (always)] pub fn imask_outrdyifg (& self) -> IMASK_OUTRDYIFG_R { IMASK_OUTRDYIFG_R :: new (((self . bits >> 3) & 1) != 0) } } impl W { # [doc = "Bit 1 - Masks COMPIFG"] # [inline (always)] # [must_use] pub fn imask_compifg (& mut self) -> IMASK_COMPIFG_W < IMASK_SPEC , 1 > { IMASK_COMPIFG_W :: new (self) } # [doc = "Bit 2 - Masks COMPINVIFG"] # [inline (always)] # [must_use] pub fn imask_compinvifg (& mut self) -> IMASK_COMPINVIFG_W < IMASK_SPEC , 2 > { IMASK_COMPINVIFG_W :: new (self) } # [doc = "Bit 3 - Masks OUTRDYIFG"] # [inline (always)] # [must_use] pub fn imask_outrdyifg (& mut self) -> IMASK_OUTRDYIFG_W < IMASK_SPEC , 3 > { IMASK_OUTRDYIFG_W :: new (self) } # [doc = r" Writes raw bits to the register."] # [doc = r""] # [doc = r" # Safety"] # [doc = r""] # [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "Interrupt mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct IMASK_SPEC ; impl crate :: RegisterSpec for IMASK_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`imask::R`](R) reader structure"] impl crate :: Readable for IMASK_SPEC { } # [doc = "`write(|w| ..)` method takes [`imask::W`](W) writer structure"] impl crate :: Writable for IMASK_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets IMASK to value 0"] impl crate :: Resettable for IMASK_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }