'''
环境配置
Mindspore ： 1.7
Python : 3.7
'''
import mindspore as ms
import mindspore.dataset as ds
import mindspore.dataset.vision.c_transforms as CV
import mindspore.dataset.transforms.c_transforms as C
import mindspore.dataset.vision.utils as mode
from mindspore.train.callback import LossMonitor, TimeMonitor
from mindspore import dtype as mstype
from nets import VGG, LeNet5

train_data_path = 'data/train'
test_data_path = 'data/test'
width = 48
height = 48
epochs = 80
batch_size = 32


def CreateData(data_path, Training=True):
    '''
    处理数据集，获取模型输入
    :param data_path:
    :param Training:
    :return:
    '''
    data_set = ds.ImageFolderDataset(data_path, num_parallel_workers=8, shuffle=True)

    # 定义数据增强算子
    if Training:
        trans = [
            CV.Decode(rgb=True),
            CV.ConvertColor(mode.ConvertMode.COLOR_RGB2GRAY),
            CV.RandomResizedCrop(size=(48, 48), scale=(0.8, 1.0), ratio=(3. / 4., 4. / 3.)),
            CV.Rescale(rescale=(1.0 / 255.0), shift=0.01),
            CV.RandomHorizontalFlip(prob=0.8),
            lambda x: x.reshape(48, 48, 1),
            CV.HWC2CHW()
        ]
    else:
        trans = [
            CV.Decode(rgb=True),
            CV.ConvertColor(mode.ConvertMode.COLOR_RGB2GRAY),
            CV.Equalize(),
            CV.Resize((width, height)),
            lambda x: x.reshape(48, 48, 1) / 255.0,
            C.TypeCast(mstype.float32),
            CV.HWC2CHW()
        ]
    type_cast_op = C.TypeCast(mstype.int32)

    data_set = data_set.map(operations=trans, input_columns="image", num_parallel_workers=8)
    data_set = data_set.map(operations=type_cast_op, input_columns=["label"])
    data_set = data_set.batch(batch_size, drop_remainder=True)
    data_set = data_set.repeat(3)
    return data_set

def TrainModel(net):
    '''
    训练网络模型
    :param net:网络结构
    :return:
    '''
    from mindspore.nn import SoftmaxCrossEntropyWithLogits
    from mindspore.nn import SGD, Momentum
    from mindspore import Model  # 承载网络结构
    from mindspore.nn.metrics import Accuracy  # 测试模型用

    # 损失函数
    net_loss = SoftmaxCrossEntropyWithLogits(sparse=True, reduction='mean')

    # 优化器
    lr = 0.01
    momentum = 0.9
    net_opt = SGD(net.trainable_params(), lr, momentum)

    # 模型
    model = Model(net, net_loss, net_opt, metrics={'accuracy': Accuracy()})
    return model

if __name__ == '__main__':

    # 网络
    net = LeNet5()

    # 回调函数
    callbacks = [
        TimeMonitor(),
        LossMonitor(),
    ]
    model = TrainModel(net)
    data_train = CreateData(train_data_path, Training=True)
    data_test = CreateData(test_data_path, Training=False)

    print('--------------Start Training--------------')
    model.train(epochs, data_train, callbacks=callbacks)
    print('--------------Finished--------------')
    acc = model.eval(data_test)  # 测试网络 {'accuracy': 0.6760463800904978}
    print('The accuracy of the test dataset is {}'.format(acc))
    ms.save_checkpoint(model, "model_saved/LeNet5.ckpt")



