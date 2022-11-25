# 定义卷积神经网络模型
import mindspore.nn as nn
from mindspore import context
from mindspore.common.initializer import Normal


class VGG(nn.Cell):
    """
    参考VGG思路设计的第一个模型
    主要注意点是感受野不能太大，以免获得很多噪声信息
    """

    def __init__(self, num_class=8, num_channel=1):
        super(VGG, self).__init__()
        self.conv1 = nn.Conv2d(1, 32, 3, stride=1, pad_mode='same')
        self.conv2 = nn.Conv2d(32, 32, 3, stride=1, pad_mode='same')
        self.conv3 = nn.Conv2d(32, 64, 3, stride=1, pad_mode='same')
        self.conv4 = nn.Conv2d(64, 64, 3, stride=1, pad_mode='same')
        self.conv5 = nn.Conv2d(64, 128, 3, stride=1, pad_mode='same')
        self.conv6 = nn.Conv2d(128, 128, 3, stride=1, pad_mode='same')

        self.max_pool2d = nn.MaxPool2d(kernel_size=2, stride=(2, 2))
        self.fc1 = nn.Dense(4608, 1024, activation='relu')
        self.fc2 = nn.Dense(1024, 128, activation='relu')
        self.fc3 = nn.Dense(128, num_class, activation='softmax')
        self.relu = nn.ReLU()
        self.flatten = nn.Flatten()
        self.dropout = nn.Dropout(keep_prob=0.5)

    def construct(self, x):
        # block1
        x = self.relu(self.conv1(x))
        x = self.relu(self.conv2(x))
        x = self.max_pool2d(x)
        x = self.dropout(x)
        # block2
        x = self.relu(self.conv3(x))
        x = self.relu(self.conv4(x))
        x = self.max_pool2d(x)
        x = self.dropout(x)
        # block3
        x = self.relu(self.conv5(x))
        x = self.relu(self.conv6(x))
        x = self.max_pool2d(x)
        x = self.dropout(x)
        # fc
        x = self.flatten(x)
        x = self.dropout(self.fc1(x))
        x = self.fc2(x)
        x = self.fc3(x)
        return x


class LeNet5(nn.Cell):
    """  LeNet5网络结构  """

    # 定义算子
    def __init__(self, num_class=8, num_channel=1):
        super(LeNet5, self).__init__()
        # 卷积层
        self.conv1 = nn.Conv2d(num_channel, 6, 5, pad_mode='valid')
        self.conv2 = nn.Conv2d(6, 16, 5, pad_mode='valid')

        # 全连接层
        self.fc1 = nn.Dense(1296, 120, weight_init=Normal(0.02))
        self.fc2 = nn.Dense(120, 84, weight_init=Normal(0.02))
        self.fc3 = nn.Dense(84, num_class, weight_init=Normal(0.02))

        # 激活函数
        self.relu = nn.ReLU()

        # 最大池化层
        self.max_pool2d = nn.MaxPool2d(kernel_size=2, stride=2)

        # 网络展开
        self.flatten = nn.Flatten()

    # 建构网络
    def construct(self, x):
        x = self.conv1(x)
        x = self.relu(x)
        x = self.max_pool2d(x)
        x = self.conv2(x)
        x = self.relu(x)
        x = self.max_pool2d(x)
        x = self.flatten(x)
        x = self.fc1(x)
        x = self.relu(x)
        x = self.fc2(x)
        x = self.relu(x)
        x = self.fc3(x)
        return x
