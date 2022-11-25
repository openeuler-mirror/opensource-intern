import os
import sys
import cv2
import time
import numpy as np
from mindspore import load_checkpoint, load_param_into_net
from mindspore import Model, Tensor  # 承载网络结构
from nets import LeNet5
import mindspore as ms

prefix = os.path.dirname(os.path.realpath(__file__))
logfile = '/home/omm/zerotrust.log'
img_num = 100
user = sys.argv[1]

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
classifier = cv2.CascadeClassifier(
    f"{prefix}/haarcascade_frontalface_default.xml")
# 加载模型参数
param_dict = load_checkpoint(f"{prefix}/model_saved/LeNet5.ckpt")
# 重新定义一个LeNet神经网络
net = LeNet5(num_class=8)
# 将参数加载到网络中
load_param_into_net(net, param_dict)
# 获取模型结构及参数
model = Model(net)


def log(msg, err=False):
    global dbname
    timestamp = time.strftime('%Y-%m-%d %H:%M:%S', time.localtime())
    log_str = f'{timestamp} [ZerotrustMs] '
    if err == False:
        log_str += f'LOG: {msg}\n'
    else:
        log_str += f'ERROR: "{msg}"\n'
    with open(logfile, 'a') as f:
        f.write(log_str)


def FaceDetect(image):
    '''
    获取图片源，实现人脸识别和表情分类
    :param image:图片源
    :return:表情分类结果及登录标识符
    '''
    emotion = None  # 设定人脸表情的初始值

    gray = cv2.cvtColor(image, cv2.COLOR_BGR2GRAY)  # 转灰度图
    median = cv2.medianBlur(gray, 5)  # 中值滤波，过滤噪声
    # 获取图片中的人脸信息
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
        log("Can not open camera!", False)
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
        log("Error!Please select the correct mode!", True)

    return emotion


if __name__ == '__main__':
    null_count = 0
    neutral_count = 0
    anomaly_count = 0
    res = 0
    log('Ready to recognize emotion of {}, now start...'.format(user))
    try:
        res_path = '/home/omm/res'
        for i in range(1, 101):
            img_path = '/home/omm/tmp/{}.jpg'.format(i)
            emotion = get_emotion(
                mode="file", img_path=img_path, res_path=res_path)

            if emotion is None:
                null_count += 1
            elif emotion == emotion_map[4] or emotion == emotion_map[5]:
                neutral_count += 1
            else:
                anomaly_count += 1
        if neutral_count / img_num > 0.7:
            res = 1
        log('Emotion recognition is completed, '\
            '{} images  were analyzed, '\
            '{} of them is/are invalid, '\
            '{} of them is/are neutral, '\
            '{} of them is/are abnormal, '\
            'and the emotion result is {}'.format(img_num, null_count, neutral_count, anomaly_count, res))
    except Exception as e:
        log('emotion analysis because of error "{}"'.format(str(e)), True)
        res = '/'
    finally:
        print(res, end='')