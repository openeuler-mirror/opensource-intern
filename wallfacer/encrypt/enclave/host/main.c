/*
 * Copyright (c) Huawei Technologies Co., Ltd. 2020. All rights reserved.
 * secGear is licensed under the Mulan PSL v2.
 * You can use this software according to the terms and conditions of the Mulan PSL v2.
 * You may obtain a copy of Mulan PSL v2 at:
 *     http://license.coscl.org.cn/MulanPSL2
 * THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND, EITHER EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR
 * PURPOSE.
 * See the Mulan PSL v2 for more details.
 */

#include <stdio.h>
#include <unistd.h>
#include <string.h>
#include <stdlib.h>
#include "enclave.h"
#include "enclave_u.h"
#define MAXLEN 64
#define idoffset 41
#define phoneoffset  74



/**
* 从文件中读取字符串，加密字符串小于32B，解密字符串（16进制）小于64B
*/
int readStrFromFile(char *filename, char *str,int offset) {
	FILE *fp = fopen(filename, "r");
	if(fp == NULL) {
		printf("打开文件出错，请确认文件存在当前目录下！\n");
		exit(0);
	}
	
	int val=1;
	char sym[4]={'C','O','P','Y'};
	char temp[150];
	memset(temp,'0',sizeof(temp));
	while(1)
	{
		fgets(temp,150,fp);
		val=strncmp(sym,temp,4);
		if(val==0)
		  break;
	}
	memset(temp,'0',sizeof(temp));
	fgets(temp,150,fp);
	char t[32];
	for(int i=offset;i<(offset+32);i++)//id:41~73,phone:74~106
	{
		if(temp[i]==' ')  break;
		t[i-offset]=temp[i];
	}
	
	strncpy(str,t,strlen(t));
	fclose(fp);
	return 0;
}

/**
 * 把字符串写进文件
 */
void writeStrToFile(unsigned char *str, char *filename,int offset) {
	FILE *fp;
	fp = fopen(filename, "r+");
	char temp[100];
	for(int i=0;i<1;)
	{
		while(fgetc(fp)!='\n')
		i++;
	}
	fprintf(fp,"\ndrop table grinfo;      \n");//写回数据时对原有数据表清除，防止表内数据不唯一出现差错
	
	int val=1;
	char sym[4]={'C','O','P','Y'};
	memset(temp,'0',sizeof(temp));
	while(1)
	{
		fgets(temp,100,fp);
		val=strncmp(sym,temp,4);
		if(val==0)
		  break;
	}
	int size=ftell (fp);//返回文件指针相对于起始位置的偏移量
	memset(temp,'0',sizeof(temp));
	fgets(temp,100,fp);
	size=size+offset;//id:offset 40,phone:offset 73
	fseek(fp,size,SEEK_SET);//根据文件指针的位置和偏移量来定位文件指针
	
	
	for(int i = 0; i < 16; i++)
	{
		fprintf(fp,"%02x",str[i]);
	}
	fclose(fp);
}


int main()
{
    int  retval = 0;
    char *path = PATH;
    unsigned char buf[80];
    int rvalue;
    char name[20];
    char psw[56];
    char fname[17];//文件名
    char info[31];//文件中要加密信息
    char cipher[64];//文件中要解密信息
    int tag;
    cc_enclave_t *context = NULL;
    cc_enclave_result_t res;
    cc_enclave_result_t rres;

    printf("Create secgear enclave\n");

    
	res = cc_enclave_create(path, AUTO_ENCLAVE_TYPE, 0, SECGEAR_DEBUG_FLAG, NULL, 0, &context);
    if (res != CC_SUCCESS) {
        printf("Create enclave error\n");
        return res;
    }
    
	//清空原内存中数据
	/*memset(fname,'0',sizeof(fname));
	memset(info,'0',sizeof(info));
	memset(cipher,'0',sizeof(cipher));*/
	

	//SM3生成存储密钥
	printf("请输入用户名:\n");//获得用户名
	gets(name);
    transname(context,name);
    printf("请输入密码 :\n");//获得用户密码
	gets(psw);
    transpsw(context,psw);
    sm3(context);
    printf("加密成功！密文如下：\n");
    res = get_string(context, &retval, buf);
    if (res != CC_SUCCESS || retval != (int)CC_SUCCESS) {
        printf("Ecall enclave error\n");
    } 
    else {
		for(int i=0;i<16;i++)
		{
			printf("%02x",buf[i]);
		}
		printf("\n");
    }
    
    system("./db.sh");//通过脚本登陆数据库获取个人信息进行复制提取工作
    

    //SM4对文件加解密
    printf("选择加密输入1，选择解密输入2：");
    scanf("%d",&tag);
    char gg=getchar();//取走缓存区的回车符
    int offset,len;
    if(tag==1){
    	printf("请输入要加密的文件名：（该文件必须和本程序在同一目录）\n");//传输要加密的用户信息
    	gets(fname);
    	printf("\n");
    	for(int i=0;i<2;i++)
    	{
    		if(i==0)
    			offset=idoffset;
    		else
    			offset=phoneoffset;
    		readStrFromFile(fname,info,offset);//取出要加密字符串
    		printf("将要加密的信息：\n");
    		puts(info);
    		len=strlen(info);//识别要机密字符串长度
    		transinfo(context,info);//传送到安全区进行加密
    		sm4crypt(context,len);
    		printf("加密成功！密文如下：\n");
    		res = get_string(context, &retval, buf);    
    		if (res != CC_SUCCESS || retval != (int)CC_SUCCESS) {
        		printf("Ecall enclave error\n");
    		} else {
				for(int i=0;i<strlen(buf);i++)
				{
					printf("%02x",buf[i]);//输出将无符号字符串以16进制形式输出
				}
				printf("\n");
    		}
    		printf("密文将写回原文件中！\n");
    		writeStrToFile(buf,fname,offset);
    		printf("第%d加密写回完成！\n\n",i+1);
    	}
    }
    else if(tag==2){
    	printf("请输入要解密的文件名：（该文件必须和本程序在同一目录）\n");
    	gets(fname);
    	printf("\n");
    	for(int i=0;i<2;i++)
    	{
    		if(i==0)
    			offset=idoffset;
    		else
    			offset=phoneoffset;
    		readStrFromFile(fname,cipher,offset);//取出要解密的字符串
    		transcipher(context,cipher);//传送到安全区解密
    		desm4crypt(context);
    		printf("第%d解密成功！明文如下：\n",i+1);
    		res = get_string(context, &retval, buf);
    		if (res != CC_SUCCESS || retval != (int)CC_SUCCESS) {
        		printf("Ecall enclave error\n");
    		} else {
    		for(int i=0;i<strlen(buf);i++)
			{
				printf("%c",buf[i]);//输出将无符号字符串以16进制形式输出
			}
			rres=get_int(context,&rvalue);
			printf("\n明文长度为：%d",rvalue);
			printf("\n\n");
    		}
    	}
    }
    
    //操作结束，将修改文件写回数据库
    system("./dbend.sh");
    
    if (context != NULL) {
        res = cc_enclave_destroy(context);
        if(res != CC_SUCCESS) {
            printf("Destroy enclave error\n");
        }
    }
    return res;
}
