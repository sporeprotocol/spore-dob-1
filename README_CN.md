# spore-dob-1

DOB1 协议旨在使用 DOB0 协议的输出作为输入参数来动态生成 SVG 图像。由于 SVG 内容采用类 XML 格式，DOB1 协议可以视为 DOB0 协议的一个特化版本，专门用于生成 SVG 内容字符串。

## 协议详情

DOB1 协议专门用于图像生成，特别是 SVG 格式的图像。该协议需要两个解码器配合使用：第一个是 DOB0 协议，负责生成文本形式的特征值；第二个是 DOB1 协议本身，用于处理上一个解码器的输出并生成 SVG 内容。

SVG 本质上是一种用 XML 格式描述图像排列的方式，因此用户需要在 SVG 内容片段中填入实际图片的 URI。这些 URI 可以是多种有效格式，例如图片 URL、图片内容的 base64 字符串，或去中心化 URI。

关于图片的去中心化存储，我们目前推荐两种方法：
* `btcfs://<tx-hash>i<image-list-index>`
* `ipfs://<ipfs-token-id>`

以下是 DOB1 协议在实际应用中的示例：

```javascript
// Spore 中的 DNA 字节流
{
    contentType: "dob/1",
    content: {
        dna: "0xefc2866a311da5b6dfcdfc4e3c22d00d024a53217ebc33855eeab1068990ed9d"
        // ..., 其他字段为可选
    },
    // 或 content: "0xefc2866a311da5b6dfcdfc4e3c22d00d024a53217ebc33855eeab1068990ed9d",
    // 或 content: ["0xefc2866a311da5b6dfcdfc4e3c22d00d024a53217ebc33855eeab1068990ed9d", ...(可选)]
    clusterId: "0x3b0e340b6c77d7b6e4f1fb2946d526ba65bfd196a27d9a7e5b6f06b82af5d07e"
}

// Cluster 中的特征映射规则示例
{
    name: "DOBs collection",
    description: {
        description: "Collection Description",
        decoders: [
            // DOB0 pattern
            {
                decoder: {
                    type: "code_hash", // 或 "type_id" 或 "type_script"
                    hash: "0x4f441345deb88edb39228e46163a8f11ac7736376af8fe5e791e194038a3ec7b", // 当 type 为 code_hash 或 type_id 时存在
                    // script: ..., 当 type 为 type_script 时存在
                },
                pattern: [
                    [
                        "Face",
                        "String",
                        0,
                        1,
                        "options",
                        ["Laugh", "Smile", "Sad", "Angry"]
                    ],
                    [
                        "Age",
                        "Number",
                        1,
                        1,
                        "range",
                        [0, 100]
                    ],
                    [
                        "BirthMonth",
                        "Number",
                        2,
                        1,
                        "options",
                        [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
                    ],
                    [
                        "Score",
                        "Number",
                        3,
                        1,
                        "rawNumber"
                    ]
                ]
            },
            // DOB1 pattern
            {
                decoder: {
                    type: "type_script", // 或 "code_hash" 或 "type_id"
                    script: {
                        code_hash: "0x00000000000000000000000000000000000000000000000000545950455f4944",
                        hash_type: "type",
                        args: "0x784e32cef202b9d4759ea96e80d806f94051e8069fd34d761f452553700138d7"
                    }
                },
                pattern: [
                    [
                        "IMAGE.0",
                        "attributes",
                        "",
                        "raw",
                        "xmlns='http://www.w3.org/2000/svg' viewBox='0 0 300 200'"
                    ],
                    [
                        "IMAGE.0",
                        "attributes",
                        "Face",
                        "options",
                        [
                            ["Laugh", "fill='#FFFF00'"],
                            ["Smile", "fill='#FF00FF'"],
                            ["Sad", "fill='#0000FF'"],
                            ["Angry", "fill='#FF0000'"],
                        ]
                    ],
                    [
                        "IMAGE.0",
                        "elements",
                        "Age",
                        "options",
                        [
                            [[0, 25], "<rect width='100' height='100' />"],
                            [[50, 75], "<rect width='100' height='100' rx='15' />"],
                            [["*"], "<rect x='25' y='25' width='50' height='50' />"],
                        ]
                    ],
                    [
                        "IMAGE.0",
                        "elements",
                        "BirthMonth",
                        "options",
                        [
                            [[1, 5], "<image href='btcfs://b2f4560f17679d3e3fca66209ac425c660d28a252ef72444c3325c6eb0364393i0' />"],
                            [[6, 9], "<image href='ipfs://QmeQ6TfqzsjJCMtYmpbyZeMxiSzQGc6Aqg6NyJTeLYrrJr' />"]
                        ]
                    ]
                ]
            }
        ]
    }
}
```

可能的输出结果如下：

```xml
<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 300 200' fill='#0000FF'>
    <rect x='25' y='25' width='50' height='50' />
    <image href='btcfs://b2f4560f17679d3e3fca66209ac425c660d28a252ef72444c3325c6eb0364393i0' />
</svg>
```

图片 href 中的 `btcfs://...` 格式无法直接被识别为图像，需要通过 JoyID 团队提供的[dob-render-sdk](https://github.com/nervina-labs/dob-render-sdk) 从 BTC 网络获取的原始图片内容并将其转换为 base64 字符串，`ipfs://...` 的处理方式类似。

## 相关仓库
1. dob-standalone-decoder-server: https://github.com/sporeprotocol/dob-decoder-standalone-server
2. spore-dob-0: https://github.com/sporeprotocol/spore-dob-0

## 最新链上信息

`code_hash`: 0xda3525549b72970b4c95f5b5749357f20d1293d335710b674f09c32f7d54b6dc

`tx_hash`:
* 测试网: 0x18c8f1d55906cf9932c5a72ae4dc039e51e41089db6829edb3f92078c6520bc8
* 主网: 0x99cc81b5e4c311519173f3f6f771dff64a2f64c97f5f724877c4352cd1b3b32c

`type_id`:
* 测试网: 0x784e32cef202b9d4759ea96e80d806f94051e8069fd34d761f452553700138d7
* 主网: 0x8892bea4405a1f077921799bc0f4516e0ebaef7aea0dfc6614a8898fb47d5372

`type_script`:
* 测试网:
```javascript
{
    "code_hash": "0x00000000000000000000000000000000000000000000000000545950455f4944",
    "hash_type": "type",
    "args": "0x784e32cef202b9d4759ea96e80d806f94051e8069fd34d761f452553700138d7"
}
```
* 主网:
```javascript
{
    "code_hash": "0x00000000000000000000000000000000000000000000000000545950455f4944",
    "hash_type": "type",
    "args": "0x8892bea4405a1f077921799bc0f4516e0ebaef7aea0dfc6614a8898fb47d5372"
}
```
如果文档未及时更新，可以到下面的文件查看最新的链上信息：
* 测试网： https://github.com/sporeprotocol/dob-decoder-standalone-server/blob/master/settings.toml#L71
* 主网： https://github.com/sporeprotocol/dob-decoder-standalone-server/blob/master/settings.mainnet.toml#L58