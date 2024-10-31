#![allow(non_camel_case_types)]

#[derive(PartialEq)]
#[repr(u32)]
pub enum ConstId {
    A0 = 1,
    B0,
    C0,
    D0,
    A1,
    B1,
    C1,
    D1,
    M0,
    M1,
    FCI,
    FXA,
    FXB,
    CLK,
    LSR,
    CE,
    DI0,
    DI1,
    WD0,
    WD1,
    WAD0,
    WAD1,
    WAD2,
    WAD3,
    WRE,
    WCK,
    F0,
    Q0,
    F1,
    Q1,
    FCO,
    OFX0,
    OFX1,
    WDO0,
    WDO1,
    WDO2,
    WDO3,
    WADO0,
    WADO1,
    WADO2,
    WADO3,
    I,
    O,
    T,
    B,
    TRELLIS_SLICE,
    TRELLIS_IO,
    DCCA,
    CLKMUX,
    LSRMUX,
    SRMODE,
    CLKI,
    CLKO,
    DP16KD,
    DIA0,
    DIA1,
    DIA2,
    DIA3,
    DIA4,
    DIA5,
    DIA6,
    DIA7,
    DIA8,
    DIA9,
    DIA10,
    DIA11,
    DIA12,
    DIA13,
    DIA14,
    DIA15,
    DIA16,
    DIA17,
    ADA0,
    ADA1,
    ADA2,
    ADA3,
    ADA4,
    ADA5,
    ADA6,
    ADA7,
    ADA8,
    ADA9,
    ADA10,
    ADA11,
    ADA12,
    ADA13,
    CEA,
    OCEA,
    CLKA,
    WEA,
    CSA2,
    CSA1,
    CSA0,
    RSTA,
    DIB0,
    DIB1,
    DIB2,
    DIB3,
    DIB4,
    DIB5,
    DIB6,
    DIB7,
    DIB8,
    DIB9,
    DIB10,
    DIB11,
    DIB12,
    DIB13,
    DIB14,
    DIB15,
    DIB16,
    DIB17,
    ADB0,
    ADB1,
    ADB2,
    ADB3,
    ADB4,
    ADB5,
    ADB6,
    ADB7,
    ADB8,
    ADB9,
    ADB10,
    ADB11,
    ADB12,
    ADB13,
    CEB,
    OCEB,
    CLKB,
    WEB,
    CSB2,
    CSB1,
    CSB0,
    RSTB,
    DOA0,
    DOA1,
    DOA2,
    DOA3,
    DOA4,
    DOA5,
    DOA6,
    DOA7,
    DOA8,
    DOA9,
    DOA10,
    DOA11,
    DOA12,
    DOA13,
    DOA14,
    DOA15,
    DOA16,
    DOA17,
    DOB0,
    DOB1,
    DOB2,
    DOB3,
    DOB4,
    DOB5,
    DOB6,
    DOB7,
    DOB8,
    DOB9,
    DOB10,
    DOB11,
    DOB12,
    DOB13,
    DOB14,
    DOB15,
    DOB16,
    DOB17,
    MULT18X18D,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    A8,
    A9,
    A10,
    A11,
    A12,
    A13,
    A14,
    A15,
    A16,
    A17,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
    B8,
    B9,
    B10,
    B11,
    B12,
    B13,
    B14,
    B15,
    B16,
    B17,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    C10,
    C11,
    C12,
    C13,
    C14,
    C15,
    C16,
    C17,
    SIGNEDA,
    SIGNEDB,
    SOURCEA,
    SOURCEB,
    CLK0,
    CLK1,
    CLK2,
    CLK3,
    CE0,
    CE1,
    CE2,
    CE3,
    RST0,
    RST1,
    RST2,
    RST3,
    SRIA0,
    SRIA1,
    SRIA2,
    SRIA3,
    SRIA4,
    SRIA5,
    SRIA6,
    SRIA7,
    SRIA8,
    SRIA9,
    SRIA10,
    SRIA11,
    SRIA12,
    SRIA13,
    SRIA14,
    SRIA15,
    SRIA16,
    SRIA17,
    SRIB0,
    SRIB1,
    SRIB2,
    SRIB3,
    SRIB4,
    SRIB5,
    SRIB6,
    SRIB7,
    SRIB8,
    SRIB9,
    SRIB10,
    SRIB11,
    SRIB12,
    SRIB13,
    SRIB14,
    SRIB15,
    SRIB16,
    SRIB17,
    SROA0,
    SROA1,
    SROA2,
    SROA3,
    SROA4,
    SROA5,
    SROA6,
    SROA7,
    SROA8,
    SROA9,
    SROA10,
    SROA11,
    SROA12,
    SROA13,
    SROA14,
    SROA15,
    SROA16,
    SROA17,
    SROB0,
    SROB1,
    SROB2,
    SROB3,
    SROB4,
    SROB5,
    SROB6,
    SROB7,
    SROB8,
    SROB9,
    SROB10,
    SROB11,
    SROB12,
    SROB13,
    SROB14,
    SROB15,
    SROB16,
    SROB17,
    ROA0,
    ROA1,
    ROA2,
    ROA3,
    ROA4,
    ROA5,
    ROA6,
    ROA7,
    ROA8,
    ROA9,
    ROA10,
    ROA11,
    ROA12,
    ROA13,
    ROA14,
    ROA15,
    ROA16,
    ROA17,
    ROB0,
    ROB1,
    ROB2,
    ROB3,
    ROB4,
    ROB5,
    ROB6,
    ROB7,
    ROB8,
    ROB9,
    ROB10,
    ROB11,
    ROB12,
    ROB13,
    ROB14,
    ROB15,
    ROB16,
    ROB17,
    ROC0,
    ROC1,
    ROC2,
    ROC3,
    ROC4,
    ROC5,
    ROC6,
    ROC7,
    ROC8,
    ROC9,
    ROC10,
    ROC11,
    ROC12,
    ROC13,
    ROC14,
    ROC15,
    ROC16,
    ROC17,
    P0,
    P1,
    P2,
    P3,
    P4,
    P5,
    P6,
    P7,
    P8,
    P9,
    P10,
    P11,
    P12,
    P13,
    P14,
    P15,
    P16,
    P17,
    P18,
    P19,
    P20,
    P21,
    P22,
    P23,
    P24,
    P25,
    P26,
    P27,
    P28,
    P29,
    P30,
    P31,
    P32,
    P33,
    P34,
    P35,
    SIGNEDP,
    ALU54B,
    SIGNEDIA,
    SIGNEDIB,
    SIGNEDCIN,
    A18,
    A19,
    A20,
    A21,
    A22,
    A23,
    A24,
    A25,
    A26,
    A27,
    A28,
    A29,
    A30,
    A31,
    A32,
    A33,
    A34,
    A35,
    B18,
    B19,
    B20,
    B21,
    B22,
    B23,
    B24,
    B25,
    B26,
    B27,
    B28,
    B29,
    B30,
    B31,
    B32,
    B33,
    B34,
    B35,
    C18,
    C19,
    C20,
    C21,
    C22,
    C23,
    C24,
    C25,
    C26,
    C27,
    C28,
    C29,
    C30,
    C31,
    C32,
    C33,
    C34,
    C35,
    C36,
    C37,
    C38,
    C39,
    C40,
    C41,
    C42,
    C43,
    C44,
    C45,
    C46,
    C47,
    C48,
    C49,
    C50,
    C51,
    C52,
    C53,
    CFB0,
    CFB1,
    CFB2,
    CFB3,
    CFB4,
    CFB5,
    CFB6,
    CFB7,
    CFB8,
    CFB9,
    CFB10,
    CFB11,
    CFB12,
    CFB13,
    CFB14,
    CFB15,
    CFB16,
    CFB17,
    CFB18,
    CFB19,
    CFB20,
    CFB21,
    CFB22,
    CFB23,
    CFB24,
    CFB25,
    CFB26,
    CFB27,
    CFB28,
    CFB29,
    CFB30,
    CFB31,
    CFB32,
    CFB33,
    CFB34,
    CFB35,
    CFB36,
    CFB37,
    CFB38,
    CFB39,
    CFB40,
    CFB41,
    CFB42,
    CFB43,
    CFB44,
    CFB45,
    CFB46,
    CFB47,
    CFB48,
    CFB49,
    CFB50,
    CFB51,
    CFB52,
    CFB53,
    MA0,
    MA1,
    MA2,
    MA3,
    MA4,
    MA5,
    MA6,
    MA7,
    MA8,
    MA9,
    MA10,
    MA11,
    MA12,
    MA13,
    MA14,
    MA15,
    MA16,
    MA17,
    MA18,
    MA19,
    MA20,
    MA21,
    MA22,
    MA23,
    MA24,
    MA25,
    MA26,
    MA27,
    MA28,
    MA29,
    MA30,
    MA31,
    MA32,
    MA33,
    MA34,
    MA35,
    MB0,
    MB1,
    MB2,
    MB3,
    MB4,
    MB5,
    MB6,
    MB7,
    MB8,
    MB9,
    MB10,
    MB11,
    MB12,
    MB13,
    MB14,
    MB15,
    MB16,
    MB17,
    MB18,
    MB19,
    MB20,
    MB21,
    MB22,
    MB23,
    MB24,
    MB25,
    MB26,
    MB27,
    MB28,
    MB29,
    MB30,
    MB31,
    MB32,
    MB33,
    MB34,
    MB35,
    CIN0,
    CIN1,
    CIN2,
    CIN3,
    CIN4,
    CIN5,
    CIN6,
    CIN7,
    CIN8,
    CIN9,
    CIN10,
    CIN11,
    CIN12,
    CIN13,
    CIN14,
    CIN15,
    CIN16,
    CIN17,
    CIN18,
    CIN19,
    CIN20,
    CIN21,
    CIN22,
    CIN23,
    CIN24,
    CIN25,
    CIN26,
    CIN27,
    CIN28,
    CIN29,
    CIN30,
    CIN31,
    CIN32,
    CIN33,
    CIN34,
    CIN35,
    CIN36,
    CIN37,
    CIN38,
    CIN39,
    CIN40,
    CIN41,
    CIN42,
    CIN43,
    CIN44,
    CIN45,
    CIN46,
    CIN47,
    CIN48,
    CIN49,
    CIN50,
    CIN51,
    CIN52,
    CIN53,
    OP0,
    OP1,
    OP2,
    OP3,
    OP4,
    OP5,
    OP6,
    OP7,
    OP8,
    OP9,
    OP10,
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    R16,
    R17,
    R18,
    R19,
    R20,
    R21,
    R22,
    R23,
    R24,
    R25,
    R26,
    R27,
    R28,
    R29,
    R30,
    R31,
    R32,
    R33,
    R34,
    R35,
    R36,
    R37,
    R38,
    R39,
    R40,
    R41,
    R42,
    R43,
    R44,
    R45,
    R46,
    R47,
    R48,
    R49,
    R50,
    R51,
    R52,
    R53,
    CO0,
    CO1,
    CO2,
    CO3,
    CO4,
    CO5,
    CO6,
    CO7,
    CO8,
    CO9,
    CO10,
    CO11,
    CO12,
    CO13,
    CO14,
    CO15,
    CO16,
    CO17,
    CO18,
    CO19,
    CO20,
    CO21,
    CO22,
    CO23,
    CO24,
    CO25,
    CO26,
    CO27,
    CO28,
    CO29,
    CO30,
    CO31,
    CO32,
    CO33,
    CO34,
    CO35,
    CO36,
    CO37,
    CO38,
    CO39,
    CO40,
    CO41,
    CO42,
    CO43,
    CO44,
    CO45,
    CO46,
    CO47,
    CO48,
    CO49,
    CO50,
    CO51,
    CO52,
    CO53,
    EQZ,
    EQZM,
    EQOM,
    EQPAT,
    EQPATB,
    OVER,
    UNDER,
    OVERUNDER,
    SIGNEDR,
    EHXPLLL,
    CLKFB,
    PHASESEL1,
    PHASESEL0,
    PHASEDIR,
    PHASESTEP,
    PHASELOADREG,
    STDBY,
    PLLWAKESYNC,
    RST,
    ENCLKOP,
    ENCLKOS,
    ENCLKOS2,
    ENCLKOS3,
    CLKOP,
    CLKOS,
    CLKOS2,
    CLKOS3,
    LOCK,
    INTLOCK,
    REFCLK,
    CLKINTFB,
    EXTREFB,
    REFCLKP,
    REFCLKN,
    REFCLKO,
    DCUA,
    CH0_HDINP,
    CH1_HDINP,
    CH0_HDINN,
    CH1_HDINN,
    D_TXBIT_CLKP_FROM_ND,
    D_TXBIT_CLKN_FROM_ND,
    D_SYNC_ND,
    D_TXPLL_LOL_FROM_ND,
    CH0_RX_REFCLK,
    CH1_RX_REFCLK,
    CH0_FF_RXI_CLK,
    CH1_FF_RXI_CLK,
    CH0_FF_TXI_CLK,
    CH1_FF_TXI_CLK,
    CH0_FF_EBRD_CLK,
    CH1_FF_EBRD_CLK,
    CH0_FF_TX_D_0,
    CH1_FF_TX_D_0,
    CH0_FF_TX_D_1,
    CH1_FF_TX_D_1,
    CH0_FF_TX_D_2,
    CH1_FF_TX_D_2,
    CH0_FF_TX_D_3,
    CH1_FF_TX_D_3,
    CH0_FF_TX_D_4,
    CH1_FF_TX_D_4,
    CH0_FF_TX_D_5,
    CH1_FF_TX_D_5,
    CH0_FF_TX_D_6,
    CH1_FF_TX_D_6,
    CH0_FF_TX_D_7,
    CH1_FF_TX_D_7,
    CH0_FF_TX_D_8,
    CH1_FF_TX_D_8,
    CH0_FF_TX_D_9,
    CH1_FF_TX_D_9,
    CH0_FF_TX_D_10,
    CH1_FF_TX_D_10,
    CH0_FF_TX_D_11,
    CH1_FF_TX_D_11,
    CH0_FF_TX_D_12,
    CH1_FF_TX_D_12,
    CH0_FF_TX_D_13,
    CH1_FF_TX_D_13,
    CH0_FF_TX_D_14,
    CH1_FF_TX_D_14,
    CH0_FF_TX_D_15,
    CH1_FF_TX_D_15,
    CH0_FF_TX_D_16,
    CH1_FF_TX_D_16,
    CH0_FF_TX_D_17,
    CH1_FF_TX_D_17,
    CH0_FF_TX_D_18,
    CH1_FF_TX_D_18,
    CH0_FF_TX_D_19,
    CH1_FF_TX_D_19,
    CH0_FF_TX_D_20,
    CH1_FF_TX_D_20,
    CH0_FF_TX_D_21,
    CH1_FF_TX_D_21,
    CH0_FF_TX_D_22,
    CH1_FF_TX_D_22,
    CH0_FF_TX_D_23,
    CH1_FF_TX_D_23,
    CH0_FFC_EI_EN,
    CH1_FFC_EI_EN,
    CH0_FFC_PCIE_DET_EN,
    CH1_FFC_PCIE_DET_EN,
    CH0_FFC_PCIE_CT,
    CH1_FFC_PCIE_CT,
    CH0_FFC_SB_INV_RX,
    CH1_FFC_SB_INV_RX,
    CH0_FFC_ENABLE_CGALIGN,
    CH1_FFC_ENABLE_CGALIGN,
    CH0_FFC_SIGNAL_DETECT,
    CH1_FFC_SIGNAL_DETECT,
    CH0_FFC_FB_LOOPBACK,
    CH1_FFC_FB_LOOPBACK,
    CH0_FFC_SB_PFIFO_LP,
    CH1_FFC_SB_PFIFO_LP,
    CH0_FFC_PFIFO_CLR,
    CH1_FFC_PFIFO_CLR,
    CH0_FFC_RATE_MODE_RX,
    CH1_FFC_RATE_MODE_RX,
    CH0_FFC_RATE_MODE_TX,
    CH1_FFC_RATE_MODE_TX,
    CH0_FFC_DIV11_MODE_RX,
    CH1_FFC_DIV11_MODE_RX,
    CH0_FFC_RX_GEAR_MODE,
    CH1_FFC_RX_GEAR_MODE,
    CH0_FFC_TX_GEAR_MODE,
    CH1_FFC_TX_GEAR_MODE,
    CH0_FFC_DIV11_MODE_TX,
    CH1_FFC_DIV11_MODE_TX,
    CH0_FFC_LDR_CORE2TX_EN,
    CH1_FFC_LDR_CORE2TX_EN,
    CH0_FFC_LANE_TX_RST,
    CH1_FFC_LANE_TX_RST,
    CH0_FFC_LANE_RX_RST,
    CH1_FFC_LANE_RX_RST,
    CH0_FFC_RRST,
    CH1_FFC_RRST,
    CH0_FFC_TXPWDNB,
    CH1_FFC_TXPWDNB,
    CH0_FFC_RXPWDNB,
    CH1_FFC_RXPWDNB,
    CH0_LDR_CORE2TX,
    CH1_LDR_CORE2TX,
    D_SCIWDATA0,
    D_SCIWDATA1,
    D_SCIWDATA2,
    D_SCIWDATA3,
    D_SCIWDATA4,
    D_SCIWDATA5,
    D_SCIWDATA6,
    D_SCIWDATA7,
    D_SCIADDR0,
    D_SCIADDR1,
    D_SCIADDR2,
    D_SCIADDR3,
    D_SCIADDR4,
    D_SCIADDR5,
    D_SCIENAUX,
    D_SCISELAUX,
    CH0_SCIEN,
    CH1_SCIEN,
    CH0_SCISEL,
    CH1_SCISEL,
    D_SCIRD,
    D_SCIWSTN,
    D_CYAWSTN,
    D_FFC_SYNC_TOGGLE,
    D_FFC_DUAL_RST,
    D_FFC_MACRO_RST,
    D_FFC_MACROPDB,
    D_FFC_TRST,
    CH0_FFC_CDR_EN_BITSLIP,
    CH1_FFC_CDR_EN_BITSLIP,
    D_SCAN_ENABLE,
    D_SCAN_IN_0,
    D_SCAN_IN_1,
    D_SCAN_IN_2,
    D_SCAN_IN_3,
    D_SCAN_IN_4,
    D_SCAN_IN_5,
    D_SCAN_IN_6,
    D_SCAN_IN_7,
    D_SCAN_MODE,
    D_SCAN_RESET,
    D_CIN0,
    D_CIN1,
    D_CIN2,
    D_CIN3,
    D_CIN4,
    D_CIN5,
    D_CIN6,
    D_CIN7,
    D_CIN8,
    D_CIN9,
    D_CIN10,
    D_CIN11,
    CH0_HDOUTP,
    CH1_HDOUTP,
    CH0_HDOUTN,
    CH1_HDOUTN,
    D_TXBIT_CLKP_TO_ND,
    D_TXBIT_CLKN_TO_ND,
    D_SYNC_PULSE2ND,
    D_TXPLL_LOL_TO_ND,
    CH0_FF_RX_F_CLK,
    CH1_FF_RX_F_CLK,
    CH0_FF_RX_H_CLK,
    CH1_FF_RX_H_CLK,
    CH0_FF_TX_F_CLK,
    CH1_FF_TX_F_CLK,
    CH0_FF_TX_H_CLK,
    CH1_FF_TX_H_CLK,
    CH0_FF_RX_PCLK,
    CH1_FF_RX_PCLK,
    CH0_FF_TX_PCLK,
    CH1_FF_TX_PCLK,
    CH0_FF_RX_D_0,
    CH1_FF_RX_D_0,
    CH0_FF_RX_D_1,
    CH1_FF_RX_D_1,
    CH0_FF_RX_D_2,
    CH1_FF_RX_D_2,
    CH0_FF_RX_D_3,
    CH1_FF_RX_D_3,
    CH0_FF_RX_D_4,
    CH1_FF_RX_D_4,
    CH0_FF_RX_D_5,
    CH1_FF_RX_D_5,
    CH0_FF_RX_D_6,
    CH1_FF_RX_D_6,
    CH0_FF_RX_D_7,
    CH1_FF_RX_D_7,
    CH0_FF_RX_D_8,
    CH1_FF_RX_D_8,
    CH0_FF_RX_D_9,
    CH1_FF_RX_D_9,
    CH0_FF_RX_D_10,
    CH1_FF_RX_D_10,
    CH0_FF_RX_D_11,
    CH1_FF_RX_D_11,
    CH0_FF_RX_D_12,
    CH1_FF_RX_D_12,
    CH0_FF_RX_D_13,
    CH1_FF_RX_D_13,
    CH0_FF_RX_D_14,
    CH1_FF_RX_D_14,
    CH0_FF_RX_D_15,
    CH1_FF_RX_D_15,
    CH0_FF_RX_D_16,
    CH1_FF_RX_D_16,
    CH0_FF_RX_D_17,
    CH1_FF_RX_D_17,
    CH0_FF_RX_D_18,
    CH1_FF_RX_D_18,
    CH0_FF_RX_D_19,
    CH1_FF_RX_D_19,
    CH0_FF_RX_D_20,
    CH1_FF_RX_D_20,
    CH0_FF_RX_D_21,
    CH1_FF_RX_D_21,
    CH0_FF_RX_D_22,
    CH1_FF_RX_D_22,
    CH0_FF_RX_D_23,
    CH1_FF_RX_D_23,
    CH0_FFS_PCIE_DONE,
    CH1_FFS_PCIE_DONE,
    CH0_FFS_PCIE_CON,
    CH1_FFS_PCIE_CON,
    CH0_FFS_RLOS,
    CH1_FFS_RLOS,
    CH0_FFS_LS_SYNC_STATUS,
    CH1_FFS_LS_SYNC_STATUS,
    CH0_FFS_CC_UNDERRUN,
    CH1_FFS_CC_UNDERRUN,
    CH0_FFS_CC_OVERRUN,
    CH1_FFS_CC_OVERRUN,
    CH0_FFS_RXFBFIFO_ERROR,
    CH1_FFS_RXFBFIFO_ERROR,
    CH0_FFS_TXFBFIFO_ERROR,
    CH1_FFS_TXFBFIFO_ERROR,
    CH0_FFS_RLOL,
    CH1_FFS_RLOL,
    CH0_FFS_SKP_ADDED,
    CH1_FFS_SKP_ADDED,
    CH0_FFS_SKP_DELETED,
    CH1_FFS_SKP_DELETED,
    CH0_LDR_RX2CORE,
    CH1_LDR_RX2CORE,
    D_SCIRDATA0,
    D_SCIRDATA1,
    D_SCIRDATA2,
    D_SCIRDATA3,
    D_SCIRDATA4,
    D_SCIRDATA5,
    D_SCIRDATA6,
    D_SCIRDATA7,
    D_SCIINT,
    D_SCAN_OUT_0,
    D_SCAN_OUT_1,
    D_SCAN_OUT_2,
    D_SCAN_OUT_3,
    D_SCAN_OUT_4,
    D_SCAN_OUT_5,
    D_SCAN_OUT_6,
    D_SCAN_OUT_7,
    D_COUT0,
    D_COUT1,
    D_COUT2,
    D_COUT3,
    D_COUT4,
    D_COUT5,
    D_COUT6,
    D_COUT7,
    D_COUT8,
    D_COUT9,
    D_COUT10,
    D_COUT11,
    D_COUT12,
    D_COUT13,
    D_COUT14,
    D_COUT15,
    D_COUT16,
    D_COUT17,
    D_COUT18,
    D_COUT19,
    D_REFCLKI,
    D_FFS_PLOL,
    PCSCLKDIV,
    SEL2,
    SEL1,
    SEL0,
    CDIV1,
    DP16KD_REGMODE_A_NOREG_REGMODE_B_NOREG,
    DP16KD_REGMODE_A_NOREG_REGMODE_B_OUTREG,
    DP16KD_REGMODE_A_OUTREG_REGMODE_B_NOREG,
    DP16KD_REGMODE_A_OUTREG_REGMODE_B_OUTREG,
    DP16KD_WRITEMODE_A_NORMAL_WRITEMODE_B_NORMAL,
    DP16KD_WRITEMODE_A_NORMAL_WRITEMODE_B_READBEFOREWRITE,
    DP16KD_WRITEMODE_A_NORMAL_WRITEMODE_B_WRITETHROUGH,
    PIO_IOTYPE_LVCMOS12,
    PIO_IOTYPE_LVCMOS15,
    PIO_IOTYPE_LVCMOS18,
    PIO_IOTYPE_LVCMOS25,
    PIO_IOTYPE_LVCMOS33,
    PIO_IOTYPE_LVDS,
    PIO_IOTYPE_SSTL15_I,
    PIO_IOTYPE_SSTL15_II,
    PIO_IOTYPE_SSTL18_I,
    PIO_IOTYPE_SSTL18_II,
    SCCU2C,
    SDPRAME,
    SLOGICB,
    SRAMWB,
    PAD,
    PADDI,
    PADDO,
    PADDT,
    IOLOGIC,
    SIOLOGIC,
    DI,
    IOLDO,
    IOLDOD,
    IOLDOI,
    IOLTO,
    INDD,
    LOADN,
    MOVE,
    DIRECTION,
    TSDATA0,
    TXDATA0,
    TXDATA1,
    RXDATA0,
    RXDATA1,
    INFF,
    CFLAG,
    ECLK,
    TSDATA1,
    TXDATA2,
    TXDATA3,
    RXDATA2,
    RXDATA3,
    TXDATA4,
    TXDATA5,
    TXDATA6,
    RXDATA4,
    RXDATA5,
    RXDATA6,
    DQSR90,
    DQSW270,
    DQSW,
    RDPNTR0,
    RDPNTR1,
    RDPNTR2,
    WRPNTR0,
    WRPNTR1,
    WRPNTR2,
    SLIP,
    GSR,
    JTAGG,
    TCK,
    TMS,
    TDI,
    JTDO2,
    JTDO1,
    TDO,
    JTDI,
    JTCK,
    JRTI2,
    JRTI1,
    JSHIFT,
    JUPDATE,
    JRSTN,
    JCE2,
    JCE1,
    OSCG,
    OSC,
    SEDSTDBY,
    SEDGA,
    SEDENABLE,
    SEDSTART,
    SEDFRCERR,
    SEDDONE,
    SEDINPROG,
    SEDERR,
    DTR,
    STARTPULSE,
    DTROUT0,
    DTROUT1,
    DTROUT2,
    DTROUT3,
    DTROUT4,
    DTROUT5,
    DTROUT6,
    DTROUT7,
    USRMCLK,
    CLKDIVF,
    ALIGNWD,
    CDIVX,
    ECLKSYNCB,
    ECLKI,
    STOP,
    ECLKO,
    DLLDELD,
    A,
    DDRDEL,
    Z,
    DDRDLL,
    UDDCNTLN,
    FREEZE,
    DIVOSC,
    DCNTL0,
    DCNTL1,
    DCNTL2,
    DCNTL3,
    DCNTL4,
    DCNTL5,
    DCNTL6,
    DCNTL7,
    DQSBUFM,
    DQSI,
    READ1,
    READ0,
    READCLKSEL2,
    READCLKSEL1,
    READCLKSEL0,
    DYNDELAY0,
    DYNDELAY1,
    DYNDELAY2,
    DYNDELAY3,
    DYNDELAY4,
    DYNDELAY5,
    DYNDELAY6,
    DYNDELAY7,
    PAUSE,
    RDLOADN,
    RDMOVE,
    RDDIRECTION,
    WRLOADN,
    WRMOVE,
    WRDIRECTION,
    DATAVALID,
    BURSTDET,
    RDCFLAG,
    WRCFLAG,
    SCLK,
    TRELLIS_ECLKBUF,
    MULT18X18D_REGS_ALL,
    MULT18X18D_REGS_INPUT,
    MULT18X18D_REGS_NONE,
    MULT18X18D_REGS_OUTPUT,
    MULT18X18D_REGS_PIPELINE,
    P,
    ECLKBRIDGECS,
    SEL,
    ECSOUT,
    WIRE_TYPE_NONE,
    WIRE_TYPE_SLICE,
    WIRE_TYPE_DQS,
    WIRE_TYPE_IOLOGIC,
    WIRE_TYPE_SIOLOGIC,
    WIRE_TYPE_PIO,
    WIRE_TYPE_EBR,
    WIRE_TYPE_MULT18,
    WIRE_TYPE_ALU54,
    WIRE_TYPE_DDRDLL,
    WIRE_TYPE_CCLK,
    WIRE_TYPE_EXTREF,
    WIRE_TYPE_DCU,
    WIRE_TYPE_PLL,
    WIRE_TYPE_SED,
    WIRE_TYPE_OSC,
    WIRE_TYPE_JTAG,
    WIRE_TYPE_GSR,
    WIRE_TYPE_DTR,
    WIRE_TYPE_PCSCLKDIV,
    WIRE_TYPE_H00,
    WIRE_TYPE_H01,
    WIRE_TYPE_H02,
    WIRE_TYPE_H06,
    WIRE_TYPE_V00,
    WIRE_TYPE_V01,
    WIRE_TYPE_V02,
    WIRE_TYPE_V06,
    WIRE_TYPE_G_HPBX,
    WIRE_TYPE_G_VPTX,
    WIRE_TYPE_L_HPBX,
    WIRE_TYPE_R_HPBX,
    IOLOGIC_MODE_IDDRX1F,
    IOLOGIC_MODE_IDDRX2F,
    IOLOGIC_MODE_IREG,
    IOLOGIC_MODE_ODDRX1F,
    IOLOGIC_MODE_ODDRX2F,
    IOLOGIC_MODE_OREG,
    IOLOGIC_MODE_TSREG,
    DCSC,
    DCSOUT,
    MODESEL,
    ALUT,
    ASYNC_RESET_RELEASE,
    BEL,
    BLUT,
    C,
    CCU2C,
    CCU2_INJECT1_0,
    CCU2_INJECT1_1,
    CEAMUX,
    CEBMUX,
    CEIMUX,
    CEMUX,
    CEOMUX,
    CER,
    CEW,
    CH0_AUTO_CALIB_EN,
    CH0_AUTO_FACQ_EN,
    CH0_BAND_THRESHOLD,
    CH0_CALIB_CK_MODE,
    CH0_CC_MATCH_1,
    CH0_CC_MATCH_2,
    CH0_CC_MATCH_3,
    CH0_CC_MATCH_4,
    CH0_CDR_CNT4SEL,
    CH0_CDR_CNT8SEL,
    CH0_CTC_BYPASS,
    CH0_DCOATDCFG,
    CH0_DCOATDDLY,
    CH0_DCOBYPSATD,
    CH0_DCOCALDIV,
    CH0_DCOCTLGI,
    CH0_DCODISBDAVOID,
    CH0_DCOFLTDAC,
    CH0_DCOFTNRG,
    CH0_DCOIOSTUNE,
    CH0_DCOITUNE,
    CH0_DCOITUNE4LSB,
    CH0_DCOIUPDNX2,
    CH0_DCONUOFLSB,
    CH0_DCOSCALEI,
    CH0_DCOSTARTVAL,
    CH0_DCOSTEP,
    CH0_DEC_BYPASS,
    CH0_ENABLE_CG_ALIGN,
    CH0_ENC_BYPASS,
    CH0_FF_RX_F_CLK_DIS,
    CH0_FF_RX_H_CLK_EN,
    CH0_FF_TX_F_CLK_DIS,
    CH0_FF_TX_H_CLK_EN,
    CH0_GE_AN_ENABLE,
    CH0_INVERT_RX,
    CH0_INVERT_TX,
    CH0_LDR_CORE2TX_SEL,
    CH0_LDR_RX2CORE_SEL,
    CH0_LEQ_OFFSET_SEL,
    CH0_LEQ_OFFSET_TRIM,
    CH0_LSM_DISABLE,
    CH0_MATCH_2_ENABLE,
    CH0_MATCH_4_ENABLE,
    CH0_MIN_IPG_CNT,
    CH0_PCIE_EI_EN,
    CH0_PCIE_MODE,
    CH0_PCS_DET_TIME_SEL,
    CH0_PDEN_SEL,
    CH0_PRBS_ENABLE,
    CH0_PRBS_LOCK,
    CH0_PRBS_SELECTION,
    CH0_RATE_MODE_RX,
    CH0_RATE_MODE_TX,
    CH0_RCV_DCC_EN,
    CH0_REG_BAND_OFFSET,
    CH0_REG_BAND_SEL,
    CH0_REG_IDAC_EN,
    CH0_REG_IDAC_SEL,
    CH0_REQ_EN,
    CH0_REQ_LVL_SET,
    CH0_RIO_MODE,
    CH0_RLOS_SEL,
    CH0_RPWDNB,
    CH0_RTERM_RX,
    CH0_RTERM_TX,
    CH0_RXIN_CM,
    CH0_RXTERM_CM,
    CH0_RX_DCO_CK_DIV,
    CH0_RX_DIV11_SEL,
    CH0_RX_GEAR_BYPASS,
    CH0_RX_GEAR_MODE,
    CH0_RX_LOS_CEQ,
    CH0_RX_LOS_EN,
    CH0_RX_LOS_HYST_EN,
    CH0_RX_LOS_LVL,
    CH0_RX_RATE_SEL,
    CH0_RX_SB_BYPASS,
    CH0_SB_BYPASS,
    CH0_SEL_SD_RX_CLK,
    CH0_TDRV_DAT_SEL,
    CH0_TDRV_POST_EN,
    CH0_TDRV_PRE_EN,
    CH0_TDRV_SLICE0_CUR,
    CH0_TDRV_SLICE0_SEL,
    CH0_TDRV_SLICE1_CUR,
    CH0_TDRV_SLICE1_SEL,
    CH0_TDRV_SLICE2_CUR,
    CH0_TDRV_SLICE2_SEL,
    CH0_TDRV_SLICE3_CUR,
    CH0_TDRV_SLICE3_SEL,
    CH0_TDRV_SLICE4_CUR,
    CH0_TDRV_SLICE4_SEL,
    CH0_TDRV_SLICE5_CUR,
    CH0_TDRV_SLICE5_SEL,
    CH0_TPWDNB,
    CH0_TX_CM_SEL,
    CH0_TX_DIV11_SEL,
    CH0_TX_GEAR_BYPASS,
    CH0_TX_GEAR_MODE,
    CH0_TX_POST_SIGN,
    CH0_TX_PRE_SIGN,
    CH0_UC_MODE,
    CH0_UDF_COMMA_A,
    CH0_UDF_COMMA_B,
    CH0_UDF_COMMA_MASK,
    CH0_WA_BYPASS,
    CH0_WA_MODE,
    CH1_AUTO_CALIB_EN,
    CH1_AUTO_FACQ_EN,
    CH1_BAND_THRESHOLD,
    CH1_CALIB_CK_MODE,
    CH1_CC_MATCH_1,
    CH1_CC_MATCH_2,
    CH1_CC_MATCH_3,
    CH1_CC_MATCH_4,
    CH1_CDR_CNT4SEL,
    CH1_CDR_CNT8SEL,
    CH1_CTC_BYPASS,
    CH1_DCOATDCFG,
    CH1_DCOATDDLY,
    CH1_DCOBYPSATD,
    CH1_DCOCALDIV,
    CH1_DCOCTLGI,
    CH1_DCODISBDAVOID,
    CH1_DCOFLTDAC,
    CH1_DCOFTNRG,
    CH1_DCOIOSTUNE,
    CH1_DCOITUNE,
    CH1_DCOITUNE4LSB,
    CH1_DCOIUPDNX2,
    CH1_DCONUOFLSB,
    CH1_DCOSCALEI,
    CH1_DCOSTARTVAL,
    CH1_DCOSTEP,
    CH1_DEC_BYPASS,
    CH1_ENABLE_CG_ALIGN,
    CH1_ENC_BYPASS,
    CH1_FF_RX_F_CLK_DIS,
    CH1_FF_RX_H_CLK_EN,
    CH1_FF_TX_F_CLK_DIS,
    CH1_FF_TX_H_CLK_EN,
    CH1_GE_AN_ENABLE,
    CH1_INVERT_RX,
    CH1_INVERT_TX,
    CH1_LDR_CORE2TX_SEL,
    CH1_LDR_RX2CORE_SEL,
    CH1_LEQ_OFFSET_SEL,
    CH1_LEQ_OFFSET_TRIM,
    CH1_LSM_DISABLE,
    CH1_MATCH_2_ENABLE,
    CH1_MATCH_4_ENABLE,
    CH1_MIN_IPG_CNT,
    CH1_PCIE_EI_EN,
    CH1_PCIE_MODE,
    CH1_PCS_DET_TIME_SEL,
    CH1_PDEN_SEL,
    CH1_PRBS_ENABLE,
    CH1_PRBS_LOCK,
    CH1_PRBS_SELECTION,
    CH1_RATE_MODE_RX,
    CH1_RATE_MODE_TX,
    CH1_RCV_DCC_EN,
    CH1_REG_BAND_OFFSET,
    CH1_REG_BAND_SEL,
    CH1_REG_IDAC_EN,
    CH1_REG_IDAC_SEL,
    CH1_REQ_EN,
    CH1_REQ_LVL_SET,
    CH1_RIO_MODE,
    CH1_RLOS_SEL,
    CH1_RPWDNB,
    CH1_RTERM_RX,
    CH1_RTERM_TX,
    CH1_RXIN_CM,
    CH1_RXTERM_CM,
    CH1_RX_DCO_CK_DIV,
    CH1_RX_DIV11_SEL,
    CH1_RX_GEAR_BYPASS,
    CH1_RX_GEAR_MODE,
    CH1_RX_LOS_CEQ,
    CH1_RX_LOS_EN,
    CH1_RX_LOS_HYST_EN,
    CH1_RX_LOS_LVL,
    CH1_RX_RATE_SEL,
    CH1_RX_SB_BYPASS,
    CH1_SB_BYPASS,
    CH1_SEL_SD_RX_CLK,
    CH1_TDRV_DAT_SEL,
    CH1_TDRV_POST_EN,
    CH1_TDRV_PRE_EN,
    CH1_TDRV_SLICE0_CUR,
    CH1_TDRV_SLICE0_SEL,
    CH1_TDRV_SLICE1_CUR,
    CH1_TDRV_SLICE1_SEL,
    CH1_TDRV_SLICE2_CUR,
    CH1_TDRV_SLICE2_SEL,
    CH1_TDRV_SLICE3_CUR,
    CH1_TDRV_SLICE3_SEL,
    CH1_TDRV_SLICE4_CUR,
    CH1_TDRV_SLICE4_SEL,
    CH1_TDRV_SLICE5_CUR,
    CH1_TDRV_SLICE5_SEL,
    CH1_TPWDNB,
    CH1_TX_CM_SEL,
    CH1_TX_DIV11_SEL,
    CH1_TX_GEAR_BYPASS,
    CH1_TX_GEAR_MODE,
    CH1_TX_POST_SIGN,
    CH1_TX_PRE_SIGN,
    CH1_UC_MODE,
    CH1_UDF_COMMA_A,
    CH1_UDF_COMMA_B,
    CH1_UDF_COMMA_MASK,
    CH1_WA_BYPASS,
    CH1_WA_MODE,
    CIN,
    CLAMP,
    CLK0_DIV,
    CLK1_DIV,
    CLK2_DIV,
    CLK3_DIV,
    CLKAMUX,
    CLKBMUX,
    CLKFB_DIV,
    CLKIMUX,
    CLKI_DIV,
    CLKOMUX,
    CLKOP_DIV,
    CLKOP_ENABLE,
    CLKOP_TRIM_DELAY,
    CLKOP_TRIM_POL,
    CLKOS2_DIV,
    CLKOS2_ENABLE,
    CLKOS3_DIV,
    CLKOS3_ENABLE,
    CLKOS_DIV,
    CLKOS_ENABLE,
    CLKOS_TRIM_DELAY,
    CLKOS_TRIM_POL,
    CLKR,
    CLKW,
    COUT,
    CSDECODE_A,
    CSDECODE_B,
    D,
    D2,
    D3,
    D4,
    D5,
    D6,
    DATAMUX_MDDR,
    DATAMUX_ODDR,
    DATAMUX_OREG,
    DATA_WIDTH_A,
    DATA_WIDTH_B,
    DATA_WIDTH_W,
    DCSMODE,
    DDRDLLA,
    DELAYF,
    DELAYG,
    DEL_MODE,
    DEL_VALUE,
    DIFFRESISTOR,
    DIR,
    DIV,
    DPHASE_SOURCE,
    DQS_LI_DEL_ADJ,
    DQS_LI_DEL_VAL,
    DQS_LO_DEL_ADJ,
    DQS_LO_DEL_VAL,
    DRIVE,
    D_BITCLK_FROM_ND_EN,
    D_BITCLK_LOCAL_EN,
    D_BITCLK_ND_EN,
    D_BUS8BIT_SEL,
    D_CDR_LOL_SET,
    D_CMUSETBIASI,
    D_CMUSETI4CPP,
    D_CMUSETI4CPZ,
    D_CMUSETI4VCO,
    D_CMUSETICP4P,
    D_CMUSETICP4Z,
    D_CMUSETINITVCT,
    D_CMUSETISCL4VCO,
    D_CMUSETP1GM,
    D_CMUSETP2AGM,
    D_CMUSETZGM,
    D_DCO_CALIB_TIME_SEL,
    D_HIGH_MARK,
    D_IB_PWDNB,
    D_ISETLOS,
    D_LOW_MARK,
    D_MACROPDB,
    D_PD_ISET,
    D_PLL_LOL_SET,
    D_REFCK_MODE,
    D_REQ_ISET,
    D_RG_EN,
    D_RG_SET,
    D_SETICONST_AUX,
    D_SETICONST_CH,
    D_SETIRPOLY_AUX,
    D_SETIRPOLY_CH,
    D_SETPLLRC,
    D_SYNC_LOCAL_EN,
    D_SYNC_ND_EN,
    D_TXPLL_PWDNB,
    D_TX_VCO_CK_DIV,
    D_XGE_MODE,
    E,
    ECP5_IS_GLOBAL,
    ER1,
    ER2,
    FEEDBK_PATH,
    FORCE_MAX_DELAY,
    FORCE_ZERO_BARREL_SHIFT,
    FREQ_LOCK_ACCURACY,
    GND,
    HYSTERESIS,
    ICP_CURRENT,
    IDDR71B,
    IDDRX1F,
    IDDRX2DQA,
    IDDRX2F,
    INIT,
    INIT0,
    INIT1,
    INITVAL,
    INJECT1_0,
    INJECT1_1,
    INTFB_WAKE,
    INT_LOCK_STICKY,
    INV,
    IOLTOMUX,
    IO_TYPE,
    KVCO,
    L6MUX21,
    LEGACY,
    LOC,
    LPF_CAPACITOR,
    LPF_RESISTOR,
    LSRIMUX,
    LSRMODE,
    LSROMUX,
    LUT0_INITVAL,
    LUT1_INITVAL,
    LUT4,
    M,
    MASK01,
    MASKPAT,
    MASKPAT_SOURCE,
    MCPAT,
    MCPAT_SOURCE,
    MFG1_TEST,
    MFG2_TEST,
    MFG_ENABLE_FILTEROPAMP,
    MFG_EN_UP,
    MFG_FLOAT_ICP,
    MFG_FORCE_VFILTER,
    MFG_GMCREF_SEL,
    MFG_GMC_GAIN,
    MFG_GMC_PRESET,
    MFG_GMC_RESET,
    MFG_GMC_TEST,
    MFG_ICP_TEST,
    MFG_LF_PRESET,
    MFG_LF_RESET,
    MFG_LF_RESGRND,
    MODE,
    MULT_BYPASS,
    OCEAMUX,
    OCEBMUX,
    OCER,
    ODDR71B,
    ODDRX1F,
    ODDRX2DQA,
    ODDRX2DQSB,
    ODDRX2F,
    OPENDRAIN,
    OSHX2A,
    OUTDIVIDER_MUXA,
    OUTDIVIDER_MUXB,
    OUTDIVIDER_MUXC,
    OUTDIVIDER_MUXD,
    PDPW16KD,
    PFUMX,
    PLLRST_ENA,
    PLL_LOCK_MODE,
    PULLMODE,
    Q,
    Q2,
    Q3,
    Q4,
    Q5,
    Q6,
    QWL,
    REFCK_DCBIAS_EN,
    REFCK_PWDNB,
    REFCK_RTERM,
    REFIN_RESET,
    REG0_LSRMODE,
    REG0_REGSET,
    REG0_SD,
    REG1_LSRMODE,
    REG1_REGSET,
    REG1_SD,
    REGMODE,
    REGMODE_A,
    REGMODE_B,
    REGSET,
    REG_FLAG_CLK,
    REG_FLAG_CE,
    REG_FLAG_RST,
    REG_INPUTA_CE,
    REG_INPUTA_CLK,
    REG_INPUTA_RST,
    REG_INPUTB_CE,
    REG_INPUTB_CLK,
    REG_INPUTB_RST,
    REG_INPUTC0_CLK,
    REG_INPUTC0_CE,
    REG_INPUTC0_RST,
    REG_INPUTC1_CLK,
    REG_INPUTC1_CE,
    REG_INPUTC1_RST,
    REG_INPUTC_CLK,
    REG_INPUTC_CE,
    REG_INPUTC_RST,
    REG_INPUTCFB_CLK,
    REG_INPUTCFB_CE,
    REG_INPUTCFB_RST,
    REG_OPCODEIN_0_CE,
    REG_OPCODEIN_0_CLK,
    REG_OPCODEIN_0_RST,
    REG_OPCODEIN_1_CE,
    REG_OPCODEIN_1_CLK,
    REG_OPCODEIN_1_RST,
    REG_OPCODEOP0_0_CE,
    REG_OPCODEOP0_0_CLK,
    REG_OPCODEOP0_0_RST,
    REG_OPCODEOP0_1_CE,
    REG_OPCODEOP0_1_CLK,
    REG_OPCODEOP0_1_RST,
    REG_OPCODEOP1_0_CLK,
    REG_OPCODEOP1_1_CLK,
    REG_OUTPUT0_CLK,
    REG_OUTPUT0_CE,
    REG_OUTPUT0_RST,
    REG_OUTPUT1_CLK,
    REG_OUTPUT1_CE,
    REG_OUTPUT1_RST,
    REG_OUTPUT_CLK,
    REG_OUTPUT_CE,
    REG_OUTPUT_RST,
    REG_PIPELINE_CE,
    REG_PIPELINE_CLK,
    REG_PIPELINE_RST,
    HIGHSPEED_CLK,
    RESETMODE,
    RNDPAT,
    RSTAMUX,
    RSTBMUX,
    S0,
    S1,
    SD,
    SGSR,
    SLEWRATE,
    SOURCEB_MODE,
    STDBY_ENABLE,
    SYNCMODE,
    SYNC_ENABLE,
    T0,
    T1,
    TERMINATION,
    TILE_WIRE_ID,
    TRELLIS_DPR16X4,
    TRELLIS_FF,
    TRIMUX_TSREG,
    TSHX2DQA,
    TSHX2DQSA,
    USRMCLKI,
    USRMCLKO,
    USRMCLKTS,
    VCC,
    WCKMUX,
    WEAMUX,
    WEBMUX,
    WID,
    WREMUX,
    WRITEMODE_A,
    WRITEMODE_B,
    Y,
    ioff_dir,
    lfe5u_12f,
    lfe5u_25f,
    lfe5u_45f,
    lfe5u_85f,
    lfe5um5g_25f,
    lfe5um5g_45f,
    lfe5um5g_85f,
    lfe5um_25f,
    lfe5um_45f,
    lfe5um_85f,
    noglobal,
    pack,
    place,
    placer,
    route,
    router,
    syn_useioff,
    TRELLIS_COMB,
    TRELLIS_RAMW,
    TRELLIS_COMB_CARRY0,
    TRELLIS_COMB_CARRY1,
    WD,
    OFX,
    F,
    CCU2_INJECT1,
}