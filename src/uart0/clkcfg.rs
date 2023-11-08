# [doc = "Register `CLKCFG` reader"] pub type R = crate :: R < CLKCFG_SPEC > ; # [doc = "Register `CLKCFG` writer"] pub type W = crate :: W < CLKCFG_SPEC > ; # [doc = "Field `CLKCFG_BLOCKASYNC` reader - Async Clock Request is blocked from starting SYSOSC or forcing bus clock to 32MHz"] pub type CLKCFG_BLOCKASYNC_R = crate :: BitReader < CLKCFG_BLOCKASYNC_A > ; # [doc = "Async Clock Request is blocked from starting SYSOSC or forcing bus clock to 32MHz\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum CLKCFG_BLOCKASYNC_A { # [doc = "0: DISABLE"] CLKCFG_BLOCKASYNC_DISABLE = 0 , # [doc = "1: ENABLE"] CLKCFG_BLOCKASYNC_ENABLE = 1 , } impl From < CLKCFG_BLOCKASYNC_A > for bool { # [inline (always)] fn from (variant : CLKCFG_BLOCKASYNC_A) -> Self { variant as u8 != 0 } } impl CLKCFG_BLOCKASYNC_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> CLKCFG_BLOCKASYNC_A { match self . bits { false => CLKCFG_BLOCKASYNC_A :: CLKCFG_BLOCKASYNC_DISABLE , true => CLKCFG_BLOCKASYNC_A :: CLKCFG_BLOCKASYNC_ENABLE , } } # [doc = "DISABLE"] # [inline (always)] pub fn is_clkcfg_blockasync_disable (& self) -> bool { * self == CLKCFG_BLOCKASYNC_A :: CLKCFG_BLOCKASYNC_DISABLE } # [doc = "ENABLE"] # [inline (always)] pub fn is_clkcfg_blockasync_enable (& self) -> bool { * self == CLKCFG_BLOCKASYNC_A :: CLKCFG_BLOCKASYNC_ENABLE } } # [doc = "Field `CLKCFG_BLOCKASYNC` writer - Async Clock Request is blocked from starting SYSOSC or forcing bus clock to 32MHz"] pub type CLKCFG_BLOCKASYNC_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , CLKCFG_BLOCKASYNC_A > ; impl < 'a , REG , const O : u8 > CLKCFG_BLOCKASYNC_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "DISABLE"] # [inline (always)] pub fn clkcfg_blockasync_disable (self) -> & 'a mut crate :: W < REG > { self . variant (CLKCFG_BLOCKASYNC_A :: CLKCFG_BLOCKASYNC_DISABLE) } # [doc = "ENABLE"] # [inline (always)] pub fn clkcfg_blockasync_enable (self) -> & 'a mut crate :: W < REG > { self . variant (CLKCFG_BLOCKASYNC_A :: CLKCFG_BLOCKASYNC_ENABLE) } } # [doc = "KEY to Allow State Change -- 0xA9\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum CLKCFG_KEY_AW { # [doc = "169: _UNLOCK_W_"] CLKCFG_KEY_UNLOCK = 169 , } impl From < CLKCFG_KEY_AW > for u8 { # [inline (always)] fn from (variant : CLKCFG_KEY_AW) -> Self { variant as _ } } impl crate :: FieldSpec for CLKCFG_KEY_AW { type Ux = u8 ; } # [doc = "Field `CLKCFG_KEY` writer - KEY to Allow State Change -- 0xA9"] pub type CLKCFG_KEY_W < 'a , REG , const O : u8 > = crate :: FieldWriter < 'a , REG , 8 , O , CLKCFG_KEY_AW > ; impl < 'a , REG , const O : u8 > CLKCFG_KEY_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , REG :: Ux : From < u8 > { # [doc = "_UNLOCK_W_"] # [inline (always)] pub fn clkcfg_key_unlock (self) -> & 'a mut crate :: W < REG > { self . variant (CLKCFG_KEY_AW :: CLKCFG_KEY_UNLOCK) } } impl R { # [doc = "Bit 8 - Async Clock Request is blocked from starting SYSOSC or forcing bus clock to 32MHz"] # [inline (always)] pub fn clkcfg_blockasync (& self) -> CLKCFG_BLOCKASYNC_R { CLKCFG_BLOCKASYNC_R :: new (((self . bits >> 8) & 1) != 0) } } impl W { # [doc = "Bit 8 - Async Clock Request is blocked from starting SYSOSC or forcing bus clock to 32MHz"] # [inline (always)] # [must_use] pub fn clkcfg_blockasync (& mut self) -> CLKCFG_BLOCKASYNC_W < CLKCFG_SPEC , 8 > { CLKCFG_BLOCKASYNC_W :: new (self) } # [doc = "Bits 24:31 - KEY to Allow State Change -- 0xA9"] # [inline (always)] # [must_use] pub fn clkcfg_key (& mut self) -> CLKCFG_KEY_W < CLKCFG_SPEC , 24 > { CLKCFG_KEY_W :: new (self) } # [doc = r" Writes raw bits to the register."] # [doc = r""] # [doc = r" # Safety"] # [doc = r""] # [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "Peripheral Clock Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct CLKCFG_SPEC ; impl crate :: RegisterSpec for CLKCFG_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`clkcfg::R`](R) reader structure"] impl crate :: Readable for CLKCFG_SPEC { } # [doc = "`write(|w| ..)` method takes [`clkcfg::W`](W) writer structure"] impl crate :: Writable for CLKCFG_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets CLKCFG to value 0"] impl crate :: Resettable for CLKCFG_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }