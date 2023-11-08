# [doc = "Register `CTL` reader"] pub type R = crate :: R < CTL_SPEC > ; # [doc = "Register `CTL` writer"] pub type W = crate :: W < CTL_SPEC > ; # [doc = "Field `CTL_ENABLE` reader - OAxn Enable."] pub type CTL_ENABLE_R = crate :: BitReader < CTL_ENABLE_A > ; # [doc = "OAxn Enable.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum CTL_ENABLE_A { # [doc = "0: OFF"] CTL_ENABLE_OFF = 0 , # [doc = "1: ON"] CTL_ENABLE_ON = 1 , } impl From < CTL_ENABLE_A > for bool { # [inline (always)] fn from (variant : CTL_ENABLE_A) -> Self { variant as u8 != 0 } } impl CTL_ENABLE_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> CTL_ENABLE_A { match self . bits { false => CTL_ENABLE_A :: CTL_ENABLE_OFF , true => CTL_ENABLE_A :: CTL_ENABLE_ON , } } # [doc = "OFF"] # [inline (always)] pub fn is_ctl_enable_off (& self) -> bool { * self == CTL_ENABLE_A :: CTL_ENABLE_OFF } # [doc = "ON"] # [inline (always)] pub fn is_ctl_enable_on (& self) -> bool { * self == CTL_ENABLE_A :: CTL_ENABLE_ON } } # [doc = "Field `CTL_ENABLE` writer - OAxn Enable."] pub type CTL_ENABLE_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , CTL_ENABLE_A > ; impl < 'a , REG , const O : u8 > CTL_ENABLE_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "OFF"] # [inline (always)] pub fn ctl_enable_off (self) -> & 'a mut crate :: W < REG > { self . variant (CTL_ENABLE_A :: CTL_ENABLE_OFF) } # [doc = "ON"] # [inline (always)] pub fn ctl_enable_on (self) -> & 'a mut crate :: W < REG > { self . variant (CTL_ENABLE_A :: CTL_ENABLE_ON) } } impl R { # [doc = "Bit 0 - OAxn Enable."] # [inline (always)] pub fn ctl_enable (& self) -> CTL_ENABLE_R { CTL_ENABLE_R :: new ((self . bits & 1) != 0) } } impl W { # [doc = "Bit 0 - OAxn Enable."] # [inline (always)] # [must_use] pub fn ctl_enable (& mut self) -> CTL_ENABLE_W < CTL_SPEC , 0 > { CTL_ENABLE_W :: new (self) } # [doc = r" Writes raw bits to the register."] # [doc = r""] # [doc = r" # Safety"] # [doc = r""] # [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct CTL_SPEC ; impl crate :: RegisterSpec for CTL_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`ctl::R`](R) reader structure"] impl crate :: Readable for CTL_SPEC { } # [doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"] impl crate :: Writable for CTL_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets CTL to value 0"] impl crate :: Resettable for CTL_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }