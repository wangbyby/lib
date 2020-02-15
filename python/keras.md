# 猫狗大战
即使用keras库训练模型,然后识别猫狗图片/照片
# keras简介
Keras 是一个用 Python 编写的高级神经网络 API，它能够以 TensorFlow, CNTK, 或者 Theano 作为后端运行。Keras 的开发重点是支持快速的实验。能够以最小的时延把你的想法转换为实验结果，是做好研究的关键。
如果你在以下情况下需要深度学习库，请使用 Keras：
    允许简单而快速的原型设计（由于用户友好，高度模块化，可扩展性）。
    同时支持卷积神经网络和循环神经网络，以及两者的组合。
    在 CPU 和 GPU 上无缝运行。
**Keras 兼容的 Python 版本: Python 2.7-3.6。**
[keras文档](https://keras.io/zh/)
# Python中安装keras库
    - 先前准备
        在安装Keras之前，请安装以下后端之一：TensorFlow，Theano，或者 CNTK。推荐使用TensorFlow后端。
    - 安装TensorFlow
        ```bash
        pip install --upgrade --ignore-installed tensorflow
        ```
        下载的很慢,耐心等待
    - 确认Tensorflow安装成功
    - 安装keras
        ```bash
        pip install keras
        ```
    - 验证安装


# keras实战
**猫狗大战**
* [训练集以及测试集连接](https://pan.baidu.com/s/1EGgAAyEyEt5BPfNodaq6Rg)
* 训练集说明
    大概是一万多张图片,我只用了不到一半的图片来训练

* 项目功能
    先训练,然后识别猫狗图片
* 项目目录
* 训练结果
* 示例程序
```python
#utf-8
from __future__ import print_function
import keras
from keras.models import Sequential, load_model
from keras.layers import Dense, Dropout, Flatten
from keras.layers import Conv2D, MaxPooling2D
from keras.preprocessing import  image
import  numpy as np
import matplotlib.pyplot as plt
from PIL import Image
import os
import random
import matplotlib as mpl
from matplotlib.image import imread


batch_size = 128
num_classes = 10
epochs = 12#次数
img_rows, img_cols = 28, 28
classs1 = {1:'狗',0:'猫'}
classes2={'cat':0,'dog':1}
"""
train文件夹下
    0 : 猫🐱
    1 : 狗🐕
"""
#文件转换为 np 数组 用于predict
def image_file_nparray(image_path, width=100,height=100):
    img = image.load_img(image_path, target_size=(width,height))
    img = image.img_to_array(img)
    x = np.expand_dims(img, axis=0)
    return x
#裁剪单个图片
def convert_one_image(file_path,width=100,height=100):
    img = Image.open(file_path)
    try:
        new_img = img.resize((width,height), Image.BILINEAR)
        new_img.save(file_path)
    except Exception as e:
        print("convert image error")
        print(e)
#剪裁改目录下所有图片
#用于二级目录
def convert_all_images(all_images_path):
    first_dir = os.listdir(all_images_path)

    for i in first_dir:
        _file_path = all_images_path + '/' + i
        if os.path.isdir(_file_path): #是目录    
            file_list = os.listdir(_file_path)
            for j in file_list:
                path_image = _file_path + '/' + j
                convert_one_image(path_image)  # 转换
        else: #不是目录, 直接转换
            convert_one_image(_file_path)  # 转换
# image转换为 np 数组, 用于 训练模型
def images_nparray(all_images_path):
    import os
    _x = []
    _y = []
    for i in os.listdir(all_images_path):
        _file_path = all_images_path+'/'+i
        if os.path.isdir(_file_path):#是目录

            for j in os.listdir(_file_path):
                y_label = j.split('_')[0]
                _y.append(y_label)

                path_image = _file_path +'/'+ j

                #图片到np数组
                img__ = Image.open(path_image).convert('RGB')
                img__ = np.array(img__)
                _x.append(img__)

        #不是目录
        else:
            y_label = i.split('.')[0]
            y_label = classes2[y_label]
            print(y_label)
            _y.append(y_label)
            path_image = _file_path
            img__ = Image.open(path_image).convert('RGB')
            img__ = np.array(img__)
            _x.append(img__)
    return np.array(_x), np.array(_y)
# np数组 转换为 npz文件
def nparray_to_npz(train_path, test_path,npz_file_path):
    convert_all_images(train_path)
    convert_all_images(test_path)
    x_train,y_train = images_nparray(train_path)
    x_test,y_test = images_nparray(test_path)
    #保存压缩文件
    np.savez(npz_file_path, x_train=x_train,y_train=y_train,x_test=x_test,y_test=y_test)
#训练函数
def running_train_test(npz_file_path, save_file_path):
    input_shape = (100,100,3)
    # the data, split between train and test sets
    data = np.load(npz_file_path)
    x_train, y_train, x_test, y_test = data['x_train'],data['y_train'],data['x_test'],data['y_test']

    x_train = x_train.astype('float32')
    x_test = x_test.astype('float32')
    x_train /= 255
    x_test /= 255
    print('x_train shape:', x_train.shape)
    print(x_train.shape[0], 'train samples')
    print(x_test.shape[0], 'test samples')
    # convert class vectors to binary class matrices
    y_train = keras.utils.to_categorical(y_train, num_classes)
    y_test = keras.utils.to_categorical(y_test, num_classes)

    model = Sequential()
    model.add(Conv2D(32, kernel_size=(3, 3),
                        activation='relu',
                        input_shape=input_shape))
    model.add(Conv2D(64, (3, 3), activation='relu'))
    model.add(MaxPooling2D(pool_size=(2, 2)))
    model.add(Dropout(0.25))
    model.add(Flatten())
    model.add(Dense(128, activation='relu'))
    model.add(Dropout(0.5))
    model.add(Dense(num_classes, activation='softmax'))

    model.compile(loss=keras.losses.categorical_crossentropy,
                    optimizer=keras.optimizers.Adadelta(),
                    metrics=['accuracy'])


    hist = model.fit(x_train, y_train,
                batch_size=batch_size,
                epochs=epochs,
                verbose=1,
                validation_data=(x_test, y_test))
    score = model.evaluate(x_test, y_test, verbose=0)
    print('Test loss:', score[0])
    print('Test accuracy:', score[1])
    model.save(save_file_path) #保存
    #绘图
    plt.figure()
    print(hist.history)
    acc = hist.history['accuracy'] # 请注意history的结构有没有变化
    val_acc = hist.history['val_accuracy']
    loss = hist.history['loss']
    val_loss = hist.history['val_loss']
    epochs2 = range(len(acc))
    plt.plot(epochs2, acc, 'bo', label='Training acc')  # 'bo'为画蓝色圆点，不连线
    plt.plot(epochs2, val_acc, 'b', label='Validation acc')
    plt.title('Training and validation accuracy')
    plt.legend()  # 绘制图例，默认在右上角
    plt.figure()
    plt.plot(epochs2, loss, 'bo', label='Training loss')
    plt.plot(epochs2, val_loss, 'b', label='Validation loss')
    plt.title('Training and validation loss')
    plt.legend()

    plt.show()

# 转换与训练
def train_test_images_to_model(train, test, npz_path, h5_path):
    nparray_to_npz(train,test,npz_path)
    running_train_test(npz_path,h5_path)
# 用于预测
def load_model_from_h5_and_predict(h5_path, pre_path):
    model = load_model(h5_path)
    # convert_one_image(pre_path)
    pre_nparray = image_file_nparray(pre_path)
    pre_y = model.predict_classes(pre_nparray)
    print("结果 : ",classs1[pre_y[0]])
    return classs1[pre_y[0]]

def predict_one_file(model, pre_path):
    pre_nparray = image_file_nparray(pre_path)
    pre_y = model.predict_classes(pre_nparray)
    print("结果 : ", classs1[pre_y[0]])
    return classs1[pre_y[0]]
def predict_dir_images(h5_path, pre_path, num=10):
    model = load_model(h5_path)
    res = []
    mpl.rcParams['font.sans-serif'] = ['FangSong']  # 指定默认字体
    mpl.rcParams['axes.unicode_minus'] = False  # 解决保存图像是负号'-'显示为方块的问题
    for i in range  (num):
        imgpath =  random.randint(1,10000)
        imgpath = pre_path+"/"+str(imgpath)+".jpg"
        tmp = predict_one_file(model,imgpath)
        res.append({"file":imgpath,"result":tmp})
        img = imread(imgpath)
        plt.subplot(2,5,i+1)
        plt.title(imgpath+" : "+tmp)
        plt.imshow(img)
    return res


#main函数
if "__main__" == __name__:
    npz_path = 'cat_dog.npz'
    save_path = 'save.h5'
    test_dir_path = 'test'
    train_dir_path = 'train'
    pre = 'pre'
    #1. 要训练数据时,执行train_test_images_to_model函数
    # train_test_images_to_model(train_dir_path, test_dir_path, npz_path, save_path)
    #2. 识别图片时,执行predict_dir_images函数
    res = predict_dir_images(save_path,pre)
    plt.show()
    print(res)

```
* 参考资料
	1. kaggle数据集
数据集分为 训练集和测试集, 数据集是自己从网上找的. Kaggle的下载速度太慢,所以用百度网盘替代.
		网址 : https://pan.baidu.com/s/1EGgAAyEyEt5BPfNodaq6Rg
	2. 参考资料
		- 利用keras进行图像识别——训练自己的数据集, https://blog.csdn.net/yh_1021/article/details/83041275
		- 	【深度学习框架Keras】在小数据集上训练图片分类模型的技巧https://blog.csdn.net/bqw18744018044/article/details/83656320
		- 	Keras：自建数据集图像分类的模型训练、保存与恢复, https://blog.csdn.net/akadiao/article/details/80456742