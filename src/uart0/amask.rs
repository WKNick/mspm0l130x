# [doc = "Register `AMASK` reader"] pub type R = crate :: R < AMASK_SPEC > ; # [doc = "Register `AMASK` writer"] pub type W = crate :: W < AMASK_SPEC > ; # [doc = "Field `AMASK_VALUE` reader - Self Address Mask for 9-Bit Mode This field contains the address mask that creates a set of addresses that should be matched. A 0 bit in the MSK bitfield configures, that the corresponding bit in the ADDR bitfield of the UART9BITADDR register is don't care. A 1 bit in the MSK bitfield configures, that the corresponding bit in the ADDR bitfield of the UART9BITADDR register must match."] pub type AMASK_VALUE_R = crate :: FieldReader ; # [doc = "Field `AMASK_VALUE` writer - Self Address Mask for 9-Bit Mode This field contains the address mask that creates a set of addresses that should be matched. A 0 bit in the MSK bitfield configures, that the corresponding bit in the ADDR bitfield of the UART9BITADDR register is don't care. A 1 bit in the MSK bitfield configures, that the corresponding bit in the ADDR bitfield of the UART9BITADDR register must match."] pub type AMASK_VALUE_W < 'a , REG , const O : u8 > = crate :: FieldWriter < 'a , REG , 8 , O > ; impl R { # [doc = "Bits 0:7 - Self Address Mask for 9-Bit Mode This field contains the address mask that creates a set of addresses that should be matched. A 0 bit in the MSK bitfield configures, that the corresponding bit in the ADDR bitfield of the UART9BITADDR register is don't care. A 1 bit in the MSK bitfield configures, that the corresponding bit in the ADDR bitfield of the UART9BITADDR register must match."] # [inline (always)] pub fn amask_value (& self) -> AMASK_VALUE_R { AMASK_VALUE_R :: new ((self . bits & 0xff) as u8) } } impl W { # [doc = "Bits 0:7 - Self Address Mask for 9-Bit Mode This field contains the address mask that creates a set of addresses that should be matched. A 0 bit in the MSK bitfield configures, that the corresponding bit in the ADDR bitfield of the UART9BITADDR register is don't care. A 1 bit in the MSK bitfield configures, that the corresponding bit in the ADDR bitfield of the UART9BITADDR register must match."] # [inline (always)] # [must_use] pub fn amask_value (& mut self) -> AMASK_VALUE_W < AMASK_SPEC , 0 > { AMASK_VALUE_W :: new (self) } # [doc = r" Writes raw bits to the register."] # [doc = r""] # [doc = r" # Safety"] # [doc = r""] # [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "Self Address Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`amask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`amask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct AMASK_SPEC ; impl crate :: RegisterSpec for AMASK_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`amask::R`](R) reader structure"] impl crate :: Readable for AMASK_SPEC { } # [doc = "`write(|w| ..)` method takes [`amask::W`](W) writer structure"] impl crate :: Writable for AMASK_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets AMASK to value 0xff"] impl crate :: Resettable for AMASK_SPEC { const RESET_VALUE : Self :: Ux = 0xff ; }