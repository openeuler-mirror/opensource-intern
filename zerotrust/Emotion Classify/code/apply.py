import os
import cv2
import numpy as np
from mindspore import load_checkpoint, load_param_into_net
from mindspore import Model, Tensor  # 承载网络结构
from nets import LeNet5
import mindspore as ms

emotion_map = {
    0: 'Angry',
    1: 'Contempt',
    2: 'Disgust',
    3: 'Fear',
    4: 'Happy',
    5: 'Neutral',
    6: 'Sad',
    7: 'Surprise'
}
# 加载人脸分类器
classifier = cv2.CascadeClassifier("../haarcascade_frontalface_default.xml")
# 加载模型参数
param_dict = load_checkpoint("../model_saved/LeNet5.ckpt")
# 重新定义一个LeNet神经网络
net = LeNet5(num_class=8)
# 将参数加载到网络中
load_param_into_net(net, param_dict)
# 获取模型结构及参数
model = Model(net)


def FaceDetect(image):
    '''
    获取图片源，实现人脸识别和表情分类
    :param image:图片源
    :return:表情分类结果及登录标识符
    '''
    emotion = None  # 设定人脸表情的初始值

    gray = cv2.cvtColor(image, cv2.COLOR_BGR2GRAY)  # 转灰度图
    median = cv2.medianBlur(gray, 5)  # 中值滤波，过滤噪声
    #获取图片中的人脸信息
    faceRects = classifier.detectMultiScale(
        median,
        scaleFactor=1.2,
        minNeighbors=3,
        minSize=(32, 32)
    )

    if len(faceRects):  # 大于0则检测到人脸
        for faceRect in faceRects:  # 单独框出每一张人脸
            x, y, w, h = faceRect
            color = (0, 0, 255)
            # 框出人脸
            cv2.rectangle(image, (x, y), (x + h, y + w), color, 2)
            # 获取人脸源
            src = gray[y:y + w, x:x + h]
            # 裁剪，缩放至48*48
            img = cv2.resize(src, (48, 48))
            # 归一化
            img = img / 255.
            # 扩展维度
            image_x = np.expand_dims(img, axis=0)
            image_x = np.array(image_x, dtype='float32').reshape(-1, 1, 48, 48)
            image_x = Tensor(image_x, ms.float32)

            # 预测输出
            lable_y = model.predict(image_x)
            output_class = np.argmax(lable_y[0])
            emotion = emotion_map[int(output_class)]
            cv2.putText(image, emotion, (x, y - 5), cv2.FONT_HERSHEY_COMPLEX,
                        0.8, (0, 0, 255), 2)

    return emotion


def FileImage(img_path, res_path):
    abs_path = os.path.abspath(img_path)
    image = cv2.imread(abs_path)
    emotion = FaceDetect(image)
    if not os.path.exists(res_path):
        os.makedirs(res_path)
    out_name = os.path.basename(img_path)
    cv2.imwrite(res_path + '/' + out_name, image)  # 保存情绪分类结果
    return emotion


def VideoImage():
    cap = cv2.VideoCapture(0)
    if not cap.isOpened():
        print("Can not open camera!")
        exit()
    while True:
        # 逐帧捕获
        ret, frame = cap.read()
        emotion = FaceDetect(frame)
        cv2.imshow("frame", frame)
        if cv2.waitKey(1) == ord('q'):
            break
    cap.release()
    cv2.destroyAllWindows()
    return emotion

def get_emotion(mode="video", img_path=None, res_path=None):
    """
    表情识别及分析
    :param mode: 必选参数：”video“ or ”file“
    :param img_path: 待检测的图片路径（若mode=”file“，则必须填入此参数）
    :param res_path: 分类结果保存路径（若mode=”file“，则必须填入此参数）
    :return: 表情识别结果
    """
    emotion = None

    if mode == "video":
        emotion = VideoImage()
    elif mode == "file":
        emotion = FileImage(img_path, res_path)
    else:
        print("Error!Please select the correct mode!")

    print("Emotion:",emotion)
    # tag = 0 # 登录标记符
    # if emotion == 'Neutral':
    #     tag = 1
    #     print("{}情绪正常，允许登录！".format(emotion))  # 情绪正常，允许登录
    # else:
    #     print("{}情绪异常，拒绝登录！".format(emotion))  # 情绪异常，拒绝登陆
    #     tag = 0
    return emotion

if __name__ == '__main__':
    """表情识别模型测试"""

    import os

    # img_path = os.path.abspath('../input_test/1.jpg')
    # res_path = os.path.abspath('../output')
    # emotion = get_emotion(mode="file", img_path=img_path, res_path=res_path)

    emotion = get_emotion(mode="video")