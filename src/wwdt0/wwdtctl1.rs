# [doc = "Register `WWDTCTL1` reader"] pub type R = crate :: R < WWDTCTL1_SPEC > ; # [doc = "Register `WWDTCTL1` writer"] pub type W = crate :: W < WWDTCTL1_SPEC > ; # [doc = "Field `WWDTCTL1_WINSEL` reader - Close Window Select"] pub type WWDTCTL1_WINSEL_R = crate :: BitReader < WWDTCTL1_WINSEL_A > ; # [doc = "Close Window Select\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum WWDTCTL1_WINSEL_A { # [doc = "0: WIN0"] WWDTCTL1_WINSEL_WIN0 = 0 , # [doc = "1: WIN1"] WWDTCTL1_WINSEL_WIN1 = 1 , } impl From < WWDTCTL1_WINSEL_A > for bool { # [inline (always)] fn from (variant : WWDTCTL1_WINSEL_A) -> Self { variant as u8 != 0 } } impl WWDTCTL1_WINSEL_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> WWDTCTL1_WINSEL_A { match self . bits { false => WWDTCTL1_WINSEL_A :: WWDTCTL1_WINSEL_WIN0 , true => WWDTCTL1_WINSEL_A :: WWDTCTL1_WINSEL_WIN1 , } } # [doc = "WIN0"] # [inline (always)] pub fn is_wwdtctl1_winsel_win0 (& self) -> bool { * self == WWDTCTL1_WINSEL_A :: WWDTCTL1_WINSEL_WIN0 } # [doc = "WIN1"] # [inline (always)] pub fn is_wwdtctl1_winsel_win1 (& self) -> bool { * self == WWDTCTL1_WINSEL_A :: WWDTCTL1_WINSEL_WIN1 } } # [doc = "Field `WWDTCTL1_WINSEL` writer - Close Window Select"] pub type WWDTCTL1_WINSEL_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , WWDTCTL1_WINSEL_A > ; impl < 'a , REG , const O : u8 > WWDTCTL1_WINSEL_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "WIN0"] # [inline (always)] pub fn wwdtctl1_winsel_win0 (self) -> & 'a mut crate :: W < REG > { self . variant (WWDTCTL1_WINSEL_A :: WWDTCTL1_WINSEL_WIN0) } # [doc = "WIN1"] # [inline (always)] pub fn wwdtctl1_winsel_win1 (self) -> & 'a mut crate :: W < REG > { self . variant (WWDTCTL1_WINSEL_A :: WWDTCTL1_WINSEL_WIN1) } } # [doc = "KEY to allow write access to this register. Writing to this register with an incorrect key activates the WWDT error signal to the ESM. Read as 0.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum WWDTCTL1_KEY_AW { # [doc = "190: _TO_UNLOCK_W_"] WWDTCTL1_KEY_UNLOCK_W = 190 , } impl From < WWDTCTL1_KEY_AW > for u8 { # [inline (always)] fn from (variant : WWDTCTL1_KEY_AW) -> Self { variant as _ } } impl crate :: FieldSpec for WWDTCTL1_KEY_AW { type Ux = u8 ; } # [doc = "Field `WWDTCTL1_KEY` writer - KEY to allow write access to this register. Writing to this register with an incorrect key activates the WWDT error signal to the ESM. Read as 0."] pub type WWDTCTL1_KEY_W < 'a , REG , const O : u8 > = crate :: FieldWriter < 'a , REG , 8 , O , WWDTCTL1_KEY_AW > ; impl < 'a , REG , const O : u8 > WWDTCTL1_KEY_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , REG :: Ux : From < u8 > { # [doc = "_TO_UNLOCK_W_"] # [inline (always)] pub fn wwdtctl1_key_unlock_w (self) -> & 'a mut crate :: W < REG > { self . variant (WWDTCTL1_KEY_AW :: WWDTCTL1_KEY_UNLOCK_W) } } impl R { # [doc = "Bit 0 - Close Window Select"] # [inline (always)] pub fn wwdtctl1_winsel (& self) -> WWDTCTL1_WINSEL_R { WWDTCTL1_WINSEL_R :: new ((self . bits & 1) != 0) } } impl W { # [doc = "Bit 0 - Close Window Select"] # [inline (always)] # [must_use] pub fn wwdtctl1_winsel (& mut self) -> WWDTCTL1_WINSEL_W < WWDTCTL1_SPEC , 0 > { WWDTCTL1_WINSEL_W :: new (self) } # [doc = "Bits 24:31 - KEY to allow write access to this register. Writing to this register with an incorrect key activates the WWDT error signal to the ESM. Read as 0."] # [inline (always)] # [must_use] pub fn wwdtctl1_key (& mut self) -> WWDTCTL1_KEY_W < WWDTCTL1_SPEC , 24 > { WWDTCTL1_KEY_W :: new (self) } # [doc = r" Writes raw bits to the register."] # [doc = r""] # [doc = r" # Safety"] # [doc = r""] # [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "Window Watchdog Timer Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wwdtctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wwdtctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct WWDTCTL1_SPEC ; impl crate :: RegisterSpec for WWDTCTL1_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`wwdtctl1::R`](R) reader structure"] impl crate :: Readable for WWDTCTL1_SPEC { } # [doc = "`write(|w| ..)` method takes [`wwdtctl1::W`](W) writer structure"] impl crate :: Writable for WWDTCTL1_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets WWDTCTL1 to value 0"] impl crate :: Resettable for WWDTCTL1_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }