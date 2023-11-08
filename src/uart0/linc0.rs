# [doc = "Register `LINC0` reader"] pub type R = crate :: R < LINC0_SPEC > ; # [doc = "Register `LINC0` writer"] pub type W = crate :: W < LINC0_SPEC > ; # [doc = "Field `LINC0_DATA` reader - 16 Bit Capture / Compare Register Captures current LINCTR value on RXD falling edge when enabled. It can generate a DATA interrupt on capture. If compare mode is enabled (DATA_MATCH = 1) a counter match can generate a LINC0 interrupt."] pub type LINC0_DATA_R = crate :: FieldReader < u16 > ; # [doc = "Field `LINC0_DATA` writer - 16 Bit Capture / Compare Register Captures current LINCTR value on RXD falling edge when enabled. It can generate a DATA interrupt on capture. If compare mode is enabled (DATA_MATCH = 1) a counter match can generate a LINC0 interrupt."] pub type LINC0_DATA_W < 'a , REG , const O : u8 > = crate :: FieldWriter < 'a , REG , 16 , O , u16 > ; impl R { # [doc = "Bits 0:15 - 16 Bit Capture / Compare Register Captures current LINCTR value on RXD falling edge when enabled. It can generate a DATA interrupt on capture. If compare mode is enabled (DATA_MATCH = 1) a counter match can generate a LINC0 interrupt."] # [inline (always)] pub fn linc0_data (& self) -> LINC0_DATA_R { LINC0_DATA_R :: new ((self . bits & 0xffff) as u16) } } impl W { # [doc = "Bits 0:15 - 16 Bit Capture / Compare Register Captures current LINCTR value on RXD falling edge when enabled. It can generate a DATA interrupt on capture. If compare mode is enabled (DATA_MATCH = 1) a counter match can generate a LINC0 interrupt."] # [inline (always)] # [must_use] pub fn linc0_data (& mut self) -> LINC0_DATA_W < LINC0_SPEC , 0 > { LINC0_DATA_W :: new (self) } # [doc = r" Writes raw bits to the register."] # [doc = r""] # [doc = r" # Safety"] # [doc = r""] # [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "UART LIN Mode Capture 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`linc0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`linc0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct LINC0_SPEC ; impl crate :: RegisterSpec for LINC0_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`linc0::R`](R) reader structure"] impl crate :: Readable for LINC0_SPEC { } # [doc = "`write(|w| ..)` method takes [`linc0::W`](W) writer structure"] impl crate :: Writable for LINC0_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets LINC0 to value 0"] impl crate :: Resettable for LINC0_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }