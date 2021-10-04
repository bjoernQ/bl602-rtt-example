MEMORY
{
    ROM       (rx)  : ORIGIN = 0x21000000, LENGTH = 128K
    ITCM      (wxa) : ORIGIN = 0x22008000, LENGTH = 48K
    DTCM      (wxa) : ORIGIN = 0x22014000, LENGTH = 48K
    XIP_FLASH (rwx) : ORIGIN = 0x23000000, LENGTH = 16M
    WIFI_RAM  (wxa) : ORIGIN = 0x42030000, LENGTH = 112K
}

REGION_ALIAS("REGION_TEXT", XIP_FLASH);
REGION_ALIAS("REGION_RODATA", XIP_FLASH);
REGION_ALIAS("REGION_DATA", DTCM);
REGION_ALIAS("REGION_BSS", DTCM);
REGION_ALIAS("REGION_HEAP", DTCM);
REGION_ALIAS("REGION_STACK", DTCM);