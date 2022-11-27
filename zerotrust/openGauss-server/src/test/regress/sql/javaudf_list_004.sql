DROP SCHEMA bicoredata_004 CASCADE;
CREATE SCHEMA bicoredata_004;
SET CURRENT_SCHEMA = bicoredata_004;

--FUNC_052
CREATE FUNCTION VideoCloudAESDecrypt(text, text)
	RETURNS text
	AS 'com.huawei.udf.VideoCloudAESDecryptUDF.evaluate'
	LANGUAGE java;
SELECT VideoCloudAESDecrypt('+u2nOW8zwKNa2Puz/4pl+8gWh7XHiwGErYRCK+/LDihvKRGKgXK/sYskEdaLvQHgAQ==','videocloudaes');

--FUNC_053
CREATE FUNCTION AdDecryptNew(text)
	RETURNS text
	AS 'com.huawei.udf.AdDecryptNew.evaluate'
	LANGUAGE java;
SELECT AdDecryptNew('74037f9013222a1d20f2a22881426ceeff46e8d5971002a53be28962d9ec9a4f');

--FUNC_054: needs bcprov-ext-jdk15on-153.jar in $JAVA_HOME/jre/lib/ext
CREATE FUNCTION MovieVMOSDecrypt(text, text, text, text)
	RETURNS text
	AS 'com.huawei.platform.bi.udf.service.movie.MovieVMOSDecryptUDF.evaluate'
	LANGUAGE java;
--SELECT MovieVMOSDecrypt('381b566d-b9e6-8121-d211-942d1a1c2e94','BTV-W09','Android 6.0','CBC_FD374FF928DE468F9886C0F1C65D411A7E4338FCFBB62D0DA66A5DF9A790A30CDBEE233267B9CD2A80F6A6DC87C0E5DB');
--SELECT MovieVMOSDecrypt('9c9c7372-a06c-2ca2-cff2-1a781e1237c3', 'JDN-AL00', 'Android 6.0.1', 'CBC_58A328A662A8667D767F1D2732E7595CFE7C5D9620B8CFEA99FB4567228DA42D');

--FUNC_055: result should differ every time
CREATE FUNCTION AESEncrypt4AD(text)
	RETURNS text
	AS 'com.huawei.platform.bi.udf.service.hnas.AESEncrypt4ADUDF.evaluate'
	LANGUAGE java;
--SELECT AESEncrypt4AD('123');

CREATE FUNCTION AESEncrypt4AD(text, text)
	RETURNS text
	AS 'com.huawei.platform.bi.udf.service.hnas.AESEncrypt4ADUDF.evaluate'
	LANGUAGE java;
---LACK OF TESTCASE

--FUNC_056
CREATE FUNCTION AESDecrypt4AD(text)
	RETURNS text
	AS 'com.huawei.platform.bi.udf.service.hnas.AESDecrypt4ADUDF.evaluate'
	LANGUAGE java;
SELECT aesdecrypt4ad('uquVZDIEoHF1ZHzi+mTgAl07tTvQUa02ZHSpmCJOxpE=');

CREATE FUNCTION AESDecrypt4AD(text, text)
	RETURNS text
	AS 'com.huawei.platform.bi.udf.service.hnAS.AESDecrypt4ADUDF.evaluate'
	LANGUAGE java;
---LACK OF TESTCASE

--FUNC_057
CREATE FUNCTION HuaweiPayDecrypt(text)
	RETURNS text
	AS 'com.huawei.udf.HuaweiPayDecryptUDF.evaluate'
	LANGUAGE java;
SELECT HuaweiPayDecrypt('encrypt1-AES-979c03f8e480fced44b2ab428e988ba6:e82b79815370075f9c3339633c186442');

--FUNC_58: needs bcprov-ext-jdk15on-153.jar in $JAVA_HOME/jre/lib/ext
CREATE FUNCTION AES256Decrypt(text, text)
	RETURNS text
	AS 'com.huawei.udf.AES256DecryptUDF.evaluate'
	LANGUAGE java;
--SELECT AES256Decrypt('0e493becc693142f7e8f94833d46390c77a6e21a1e770ea479bc9579c7255c75','def');

--FUNC_59
CREATE FUNCTION OpenAesDecrypt(text)
	RETURNS text
	AS 'com.huawei.platform.bi.udf.service.openAlliance.AesDecryptUDF.evaluate'
	LANGUAGE java;
SELECT OpenAesDecrypt ('45128950BC0832DD45D0D82DB954D3E2697F2507458BE388F45280AECFC29A8B9906F5747E983751E727C8F3CD9AA9F75CCC2516D0636F8E7190A07A95F54A41'); 

CREATE FUNCTION OpenAesDecrypt(text, text)
	RETURNS text
	AS 'com.huawei.platform.bi.udf.service.openAlliance.AesDecryptUDF.evaluate'
	LANGUAGE java;
SELECT OpenAesDecrypt ('01e9a6fb24eb3b73e79a30ad3c6ddc8c','ads');

--FUNC_60
CREATE FUNCTION HiboardAesDecrypt(text)
	RETURNS text
	AS 'com.huawei.udf.NegativeScreenDecrypt.evaluate'
	LANGUAGE java;
SELECT  HiboardAesDecrypt('llhiF1EMO+3qtBGefg0xB7S81HG8kMoMXWiV2gP2Lc4=');

--FUNC_061
CREATE FUNCTION AESDeCryptVmall(text, text)
	RETURNS text
	AS 'com.huawei.udf.AESDeCryptVmallUDF.evaluate'
	LANGUAGE java;
SELECT AESDeCryptVmall('73260498265bf42d79a5e10586d3df7c','vmall');

--FUNC_062
CREATE FUNCTION AESCBCnopadding(text)
	RETURNS text
	AS 'com.huawei.udf.AESCBCnopadding.evaluate'
	LANGUAGE java;
SELECT AESCBCnopadding('cbc_7CFFCD04AD90D89BCCF45BED977C9E2B2CA1937CEE4C2AA99B2093BECCBC6587 ');

--FUNC_063
CREATE FUNCTION VerifyAndDecrypt(text, text)
	RETURNS text
	AS 'com.huawei.platform.bi.udf.service.hota.VerifyAndDecrypt.evaluate'
	LANGUAGE java;
SELECT VerifyAndDecrypt('93B6807E1D314F6FAB18FED05E6AD62DC6C0309EFD94B374CBFC9315392C032F0F000000B6E76C6EF29AFF144C9686DA9DA73E9E', '07b824c417057ef847db3be8071601ce'); 

--FUNC_064
CREATE FUNCTION VideoImeiDesCrypt(text)
	RETURNS text
	AS 'com.huawei.videoimei.udf.VideoImeiDesCryptUDF.evaluate'
	LANGUAGE java;
SELECT videoImeidescrypt('HMc0rp1SAKeBQtv19W6R1tgjGlsAQghm2MrcjBCTPjc=');

--FUNC_065
CREATE FUNCTION PushRsaCryptUtils(text)
	RETURNS text
	AS 'com.huawei.udf.PushRsaCryptUtils.evaluate'
	LANGUAGE java;
SELECT pushrsacryptutils ('3d7a3ba7d2e39e0b052b24c32d6b7e1cd852c8ca9a5a3352d1630becfc2bace09e38c362bf5ea1dc1200eace91f88969f6ea511ca8395ce1bf67779a30b5c257f5a872ba14d10525eae6a64b65d4c77e013cdc5feddec5b5c2075806b16c9affab102c2cb19a43de52c8916ec4c3ca6be0f68b40e18f4ab77b674a15e7fe63cb');

DROP SCHEMA javaudf_list_004 CASCADE;
