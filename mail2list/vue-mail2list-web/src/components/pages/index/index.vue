<template>
  <section>
    <Tabs value="name1">
      <TabPane :label="$t('homeInfo.introduction_title')" name="name1">
        <Card shadow class="Introduction janeCard ">
          {{$t('homeInfo.introduction_Info')}}
        </Card>
      </TabPane>
    </Tabs>
    <Tabs value="name1">
      <TabPane :label="$t('homeInfo.use_title')" name="name1">
        <Row :gutter="16" style="margin-bottom: 20px">
          <Col span="8" style="height: 250px" >
            <div class="ContentQuantity">
              <div class="statisticsImg SICutCorner">
                <img src="../../../assets/img/maillist/mail1.png" alt="mail1" />
              </div>
              <ul class="textArea">
                <li>{{$t('homeInfo.select_one_title')}}</li>
              </ul>
              <div class="leftTop20">
                <Card class="Introduction cardStyle" shadow>
                  {{$t('homeInfo.selectStatistics')}}
                </Card>
              </div>
            </div>
          </Col>
          <Col span="8" style="height: 270px">
            <div class="ContentQuantity">
              <div class="statisticsImg SIRound">
                <img src="../../../assets/img/maillist/mail2.png" alt="mail2" />
              </div>
              <ul class="textArea">
                <li>{{$t('homeInfo.select_two_title')}}</li>
              </ul>
              <div class="leftTop20">
                <Card class="Introduction cardStyle" shadow>
                  {{$t('homeInfo.SubscribeStatistics')}}
                </Card>
              </div>
            </div>
          </Col>
          <Col span="8" style="height: 340px">
            <div class="ContentQuantity">
              <div class="statisticsImg SIRrdial">
                <img src="../../../assets/img/maillist/mail3.png" alt="mail3" />
              </div>
              <ul class="textArea">
                <li>{{$t('homeInfo.select_three_title')}}</li>
              </ul>
              <div class="leftTop20">
                <Card class="Introduction cardStyle" shadow>
                  {{$t('homeInfo.ConfirmStatistics')}}
                </Card>
              </div>
            </div>
          </Col>
        </Row>
        <Row :gutter="16">
          <Col span="8" style="visibility: hidden">
            <div class="ContentQuantity"></div>
          </Col>
          <Col style="margin-top: -80px;height: 290px" span="8">
            <div class="ContentQuantity">
              <div class="statisticsImg SITilt">
                <img src="../../../assets/img/maillist/mail4.png" alt="mail4" />
              </div>
              <ul class="textArea">
                <li>{{$t('homeInfo.select_four_title')}}</li>
              </ul>
              <div class="leftTop20">
                <Card class="Introduction cardStyle" shadow>
                  {{$t('homeInfo.SucceedStatistics')}}
                </Card>
              </div>
            </div>
          </Col>
          <Col style="margin-top: -20px;height: 280px" span="8">
            <div class="ContentQuantity">
              <div class="statisticsImg SIRound">
                <img src="../../../assets/img/maillist/mail-banner.png" alt="mail-banner" />
              </div>
              <ul class="textArea">
                <li>{{$t('homeInfo.select_five_title')}}</li>
              </ul>
              <div class="leftTop20">
                <Card class="Introduction cardStyle" shadow>
                  {{$t('homeInfo.JoinStatistics')}}
                </Card>
              </div>
            </div>
          </Col>
        </Row>
      </TabPane>
    </Tabs>
  </section>
</template>

<script>
import countTo from './module/count-to'
//import size from "../../../utils/winSize";
export default {
  name: "sections",
  components: { countTo },
  props: {
    msg: String
  },
  data() {
    return {
      height: 500,
    };
  },
  computed: {
    color () {
      let color = '#5cb85c';
      if (this.percent === 100) {
        color = '#5cb85c';
      }
      return color;
    }
  },
  mounted() {
    this.height = size.PageH - 500;
  },
  methods:{
  }
};
</script>
<style lang="scss" scoped>
@import "./index.scss";
.textArea{
  float:left;
  width: 50%;
  margin-top: 20px;
  letter-spacing: -.02em;
  li{
    font-size: 28px;
    font-family: "微软雅黑";
    p{
      color: rgb(102, 102, 102);
      font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen, Ubuntu, Cantarell, "Fira Sans", "Droid Sans", "Helvetica Neue", sans-serif;
    }
  }
  li:nth-child(1){
    margin-top: 0;
  }
}
.ContentQuantity{
  width: 100%;
  height: 100%;
  min-height: 0.75rem;
  transform: scale(.97);
  background-color: #fff;
  box-sizing: border-box;
  float: left;
  border-radius: .25rem;
  box-shadow: 0 0.125rem 0.4375rem 0 rgba(0,0,0,.2);

  .statisticsImg{
    width: 100px;
    height: 100px;
    display: inline-block;
    margin: 20px;
    padding: 20px;
    float: left;
    background: rgb(232, 236, 238);
    border-radius: 15%;
    img {
      width: 100%;
      height: 100%;
    }
  }
  .SIRound{
    border-radius: 50%;
  }
  .SITilt{
    transform: rotateZ(25deg)
  }
  .textIndex{
    text-indent: 20px;
  }
  .SICutCorner{
    transform: rotateZ(-15deg);
    @mixin beveled-corners($bg, $tl:0, $tr:$tl, $br:$tl, $bl:$tr) { background: $bg; background: linear-gradient(135deg, transparent $tl, $bg 0) top left, linear-gradient(225deg, transparent $tr, $bg 0) top right, linear-gradient(-45deg, transparent $br, $bg 0) bottom right, linear-gradient(45deg, transparent $bl, $bg 0) bottom left; background-size: 50% 50%; background-repeat: no-repeat;}
    @include beveled-corners(#e8ecee, 15px, 5px);
  }
  .SIRrdial{
    transform: rotateZ(8deg);
    background: radial-gradient(circle at top left, transparent 15px, #e8ecee 0) top left, radial-gradient(circle at top right, transparent 15px, #e8ecee 0) top right, radial-gradient(circle at bottom right, transparent 15px, #e8ecee 0) bottom right, radial-gradient(circle at bottom left, transparent 15px, #e8ecee 0) bottom left;
    background-size: 50% 50%;
    background-repeat: no-repeat;
  }
}
.ContentQuantity:hover{
  cursor: pointer;
  transition: 1s;
  transform: scale(1,1);
  -ms-transform:scale(1,1);     /* IE 9 */
  -moz-transform:scale(1,1);     /* Firefox */
  -webkit-transform:scale(1,1); /* Safari 和 Chrome */
  -o-transform:scale(1,1);
}
.leftTop20{
  float: left;
  .cardStyle{
    /deep/ .ivu-card-body{
      float:left;
      background: rgba(241, 241, 246, 0.6);
      margin: 10px;
    }
  }
}
.Introduction{
  width: 100%;
  color: #666666;
  letter-spacing:1px;
  .janeCard{
    background: rgba(241, 241, 246, 0.6);
    margin-bottom: 20px
  }
}
.demo-Circle-custom{
    & h1{
        color: #3f414d;
        font-size: 28px;
        font-weight: normal;
    }
    & p{
        color: #657180;
        font-size: 14px;
        margin: 10px 0 15px;
    }
    & span{
        display: block;
        padding-top: 15px;
        color: #657180;
        font-size: 14px;
        &:before{
            content: '';
            display: block;
            width: 50px;
            height: 1px;
            margin: 0 auto;
            background: #e0e3e6;
            position: relative;
            top: -15px;
        };
    }
    & span i{
        font-style: normal;
        color: #3f414d;
    }
}
/deep/ .ivu-col-span-6{
  text-align: center;
  .alignLeft{
    text-align: left
  }
}
/deep/ .ivu-tabs-ink-bar {
     background-color: #3da375;
 }
/deep/ .ivu-tabs-nav .ivu-tabs-tab-active {
    color: #515a6e;
    font-size: 16px;
    font-weight: 600;
    padding-left: 2px;
}
</style>