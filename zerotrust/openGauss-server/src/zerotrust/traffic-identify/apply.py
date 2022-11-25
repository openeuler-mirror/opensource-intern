import numpy as np
import pandas as pd
import joblib
import mindspore as ms
import os
from net import DNN
from mindspore import Model, Tensor
from mindspore import load_checkpoint, load_param_into_net

prefix = os.path.dirname(os.path.realpath(__file__))

standard_scaler = joblib.load(f'{prefix}/model_saved/StandardScaler.save')  # 加载数据缩放器模型

param_dict = load_checkpoint(f'{prefix}/model_saved/DNN.ckpt')  # 加载模型参数
net = DNN()  # 实例化网络模型
load_param_into_net(net, param_dict)  # 将参数加载到网络中
model = Model(net)  # 获取模型结构及参数

# 将模型预测编码标签与原始类别标签进行映射
encode_label = [0, 1, 2, 3, 4]
origin_label = ['BENIGN', 'DoS GoldenEye', 'DoS Hulk', 'FTP-Patator', 'PortScan']
class_map = dict(zip(encode_label, origin_label))


def identify(input_data):
    """
    流量的分类与识别
    :param input_data: 原始输入数据，是一个含有11个数据的numpy数组，其中每一行的数据分别代表以下11个特征：
                        Bwd Packet Length Min       :数据包在反向上的最小值
                        Subflow Fwd Bytes           :子流在正向中的平均字节数
                        Total Length of Fwd Packets :正向数据包的总大小
                        Fwd Packet Length Mean      :数据包在正向上的平均大小
                        Bwd Packet Length Std       :数据包反向标准偏差大小
                        Flow Duration               :数据流持续时间
                        Flow IAT Std                :两个流之间的标准差
                        Init_Win_bytes_forward      :在正向的初始窗口中发送的字节数
                        Bwd Packets/s               :每秒中后向包的数量
                        PSH Flag Count              :带有 PSH 的包数量
                        Average Packet Size         :数据包的平均大小
    :return: 模型预测的流量类型
    """

    # 数组重塑
    data = input_data.reshape(1, 11)
    # 数据标准化处理
    data = standard_scaler.transform(data)
    # 转为mindspore张量形式作为模型输入
    data = Tensor(data, ms.float32)
    predictions = model.predict(data)
    label = class_map[int(np.argmax(predictions))]
    
    # 流量的分类与识别
    # if label == 'BENIGN':   # 正常流量
    #     print("Normal Traffic!")
    # else :                  # 异常流量
    #     print("Abnormal Traffic! Attack type:",label)
    return label == 'BENIGN'
