from mindspore import nn
from mindspore.common.initializer import Normal


class DNN(nn.Cell):
    """定义深度神经网络相关算子"""
    def __init__(self, input_dims=11, output_dims=6, dropout_ratio=0.2):
        super(DNN, self).__init__()
        self.input_dims = input_dims
        self.output_dims = output_dims
        self.dropout_ratio = dropout_ratio
        # 定义所需要的运算
        self.fc1 = nn.Dense(self.input_dims, 512, weight_init=Normal(0.02))
        self.fc2 = nn.Dense(512, 256, weight_init=Normal(0.02))
        self.fc3 = nn.Dense(256, 128, weight_init=Normal(0.02))
        self.fc4 = nn.Dense(128, 64, weight_init=Normal(0.02))
        self.fc5 = nn.Dense(64, self.output_dims, weight_init=Normal(0.02))
        self.relu = nn.ReLU()
        self.log_softmax = nn.LogSoftmax()
        self.dropout = nn.Dropout(keep_prob=dropout_ratio)

    # 使用定义好的算子构建前向网络
    def construct(self, x):

        x = self.fc1(x)
        x = self.relu(x)
        x = self.dropout(x)

        x = self.fc2(x)
        x = self.relu(x)
        x = self.dropout(x)

        x = self.fc3(x)
        x = self.relu(x)
        x = self.dropout(x)

        x = self.fc4(x)
        x = self.relu(x)
        x = self.dropout(x)

        x = self.fc5(x)
        x = self.log_softmax(x)
        return x