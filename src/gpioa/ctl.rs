# [doc = "Register `CTL` reader"] pub type R = crate :: R < CTL_SPEC > ; # [doc = "Register `CTL` writer"] pub type W = crate :: W < CTL_SPEC > ; # [doc = "Field `CTL_FASTWAKEONLY` reader - FASTWAKEONLY for the global control of fastwake"] pub type CTL_FASTWAKEONLY_R = crate :: BitReader < CTL_FASTWAKEONLY_A > ; # [doc = "FASTWAKEONLY for the global control of fastwake\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum CTL_FASTWAKEONLY_A { # [doc = "0: NOT_GLOBAL_EN"] CTL_FASTWAKEONLY_NOT_GLOBAL_EN = 0 , # [doc = "1: GLOBAL_EN"] CTL_FASTWAKEONLY_GLOBAL_EN = 1 , } impl From < CTL_FASTWAKEONLY_A > for bool { # [inline (always)] fn from (variant : CTL_FASTWAKEONLY_A) -> Self { variant as u8 != 0 } } impl CTL_FASTWAKEONLY_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> CTL_FASTWAKEONLY_A { match self . bits { false => CTL_FASTWAKEONLY_A :: CTL_FASTWAKEONLY_NOT_GLOBAL_EN , true => CTL_FASTWAKEONLY_A :: CTL_FASTWAKEONLY_GLOBAL_EN , } } # [doc = "NOT_GLOBAL_EN"] # [inline (always)] pub fn is_ctl_fastwakeonly_not_global_en (& self) -> bool { * self == CTL_FASTWAKEONLY_A :: CTL_FASTWAKEONLY_NOT_GLOBAL_EN } # [doc = "GLOBAL_EN"] # [inline (always)] pub fn is_ctl_fastwakeonly_global_en (& self) -> bool { * self == CTL_FASTWAKEONLY_A :: CTL_FASTWAKEONLY_GLOBAL_EN } } # [doc = "Field `CTL_FASTWAKEONLY` writer - FASTWAKEONLY for the global control of fastwake"] pub type CTL_FASTWAKEONLY_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , CTL_FASTWAKEONLY_A > ; impl < 'a , REG , const O : u8 > CTL_FASTWAKEONLY_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NOT_GLOBAL_EN"] # [inline (always)] pub fn ctl_fastwakeonly_not_global_en (self) -> & 'a mut crate :: W < REG > { self . variant (CTL_FASTWAKEONLY_A :: CTL_FASTWAKEONLY_NOT_GLOBAL_EN) } # [doc = "GLOBAL_EN"] # [inline (always)] pub fn ctl_fastwakeonly_global_en (self) -> & 'a mut crate :: W < REG > { self . variant (CTL_FASTWAKEONLY_A :: CTL_FASTWAKEONLY_GLOBAL_EN) } } impl R { # [doc = "Bit 0 - FASTWAKEONLY for the global control of fastwake"] # [inline (always)] pub fn ctl_fastwakeonly (& self) -> CTL_FASTWAKEONLY_R { CTL_FASTWAKEONLY_R :: new ((self . bits & 1) != 0) } } impl W { # [doc = "Bit 0 - FASTWAKEONLY for the global control of fastwake"] # [inline (always)] # [must_use] pub fn ctl_fastwakeonly (& mut self) -> CTL_FASTWAKEONLY_W < CTL_SPEC , 0 > { CTL_FASTWAKEONLY_W :: new (self) } # [doc = r" Writes raw bits to the register."] # [doc = r""] # [doc = r" # Safety"] # [doc = r""] # [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "FAST WAKE GLOBAL EN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct CTL_SPEC ; impl crate :: RegisterSpec for CTL_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`ctl::R`](R) reader structure"] impl crate :: Readable for CTL_SPEC { } # [doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"] impl crate :: Writable for CTL_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets CTL to value 0"] impl crate :: Resettable for CTL_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }