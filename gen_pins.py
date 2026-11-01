#!/usr/bin/env python3
"""cw32-bsp 引脚表生成器:从 fan2nd/cw32-hal2 的 cw32-data/af/<family>.yaml
(官方数据手册 AF 表 PDF 定位提取 + 厂商 SDK 宏交叉核对,sha256 锁定)
生成各家族 BSP 的 src/pins.rs 引脚表。

用法:把 cw32-hal2 的 cw32-data/af/cw32<family>.yaml 放到 $TEMP/pinouts/
(或用 AF_DIR 指定),然后 `python gen_pins.py`。
信号名归一:TX/RX(T 家族 SDK 命名)-> TXD/RXD。
"""
import yaml, os, io, re

AF_DIR = os.environ.get("AF_DIR", os.path.join(os.environ["TEMP"], "pinouts"))
ROOT = os.path.dirname(os.path.abspath(__file__))
FAMS = ["f003", "f020", "f030", "l010", "l011", "l012", "l031", "l052", "l083", "r031", "w031"]
# 每家族 Peripherals 收编端口(由 cw32-pacs / cw32l012-pac 盘点;PAC 生成器
# 补端口后更新此表并重新生成)
# embassy 实际可落端口(PAC Peripherals 字段 ∩ embassy gpio impl 接线;
# l052/l083 的 GPIOD/GPIOE 已接线——共享向量 handler 成对派发 C+D/E+F)
GPIO_FIELDS = {
    "f003": ["A", "B", "C"], "f020": ["A", "B", "C", "F"],
    "f030": ["A", "B", "C", "F"], "l010": ["A", "B"], "l011": ["A", "B", "C"],
    "l012": ["A", "B", "C", "F"], "l031": ["A", "B", "C", "F"],
    "l052": ["A", "B", "C", "D", "F"], "l083": ["A", "B", "C", "D", "E", "F"],
    "r031": ["A", "B", "C", "F"], "w031": ["A", "B", "C", "F"],
}
SIGS = {"UART1": ["TXD", "RXD"], "SPI1": ["SCK", "MOSI", "MISO"], "I2C1": ["SDA", "SCL"],
        "SPI": ["SCK", "MOSI", "MISO"], "I2C": ["SDA", "SCL"]}


def norm_sig(s):
    return {"TX": "TXD", "RX": "RXD"}.get(s, s)


def routes_of(fam):
    y = yaml.safe_load(open(os.path.join(AF_DIR, "af_cw32%s.yaml" % fam), encoding="utf-8"))
    out = []
    for r in y["routes"]:
        per, sig = r.get("peripheral"), norm_sig(r.get("signal", ""))
        if per in SIGS and sig in SIGS[per]:
            out.append((per, sig, r["pin"][:2].replace("P", ""), int(r["pin"][2:]),
                        r["af"], r.get("source_macro", "")))
    return out, y.get("source", {}).get("document_filename") or y.get("source", {}).get("source_id", "?")


def gen_pins(fam, rows, doc):
    groups = {}
    for per, sig, port, pin, af, macro in rows:
        groups.setdefault((per, sig), []).append((port, pin, af, macro))
    ports = ", ".join("P" + p for p in GPIO_FIELDS[fam])
    lines = [
        "//! 引脚复用路由表(自动生成,勿手改)。",
        "//!",
        "//! 数据源:fan2nd/cw32-hal2 `cw32-data/af/cw32%s.yaml` —— 官方数据手册" % fam,
        "//! 《%s》AF 表的 PDF 定位提取,与厂商 SDK GPIO 宏(表内" % doc,
        "//! `source_macro`)交叉核对。生成脚本:cw32-bsp/gen_pins.py。",
        "//!",
        "//! ⚠ 可落性:embassy 的 Peripherals 端口字段由 PAC 决定——本家族 PAC",
        "//! 收编端口:%s。标注 `[PAC]` 的组合其端口未被 PAC" % ports,
        "//! 收编为 Peripherals 字段,当前无法构造(等 PAC 生成器补端口),",
        "//! 其余组合可直接经 [`bsp_pin!`] 落地。",
        "",
    ]
    for (per, sig) in sorted(groups):
        name = per + "_" + sig
        lines.append("/// `%s` %s 的全部可用引脚组合:`(端口字母, 引脚号, AF 编号)`。" % (per, sig))
        lines.append("pub const %s: &[(&str, u8, u8)] = &[" % name)
        for port, pin, af, macro in sorted(groups[(per, sig)]):
            ok = "" if port in GPIO_FIELDS[fam] else " [PAC]"
            lines.append('    ("%s", %d, %d), // %s%s' % (port, pin, af, macro, ok))
        lines.append("];")
        lines.append("")
    return "\n".join(lines)


def first_pa_pair(rows):
    txs = [(n, af) for per, sig, p, n, af, _ in rows if sig == "TXD" and p == "A"]
    rxs = [(n, af) for per, sig, p, n, af, _ in rows if sig == "RXD" and p == "A"]
    for t in txs:
        for r in rxs:
            if t[0] != r[0]:
                return t, r
    return None


if __name__ == "__main__":
    for fam in FAMS:
        crate = "cw32%s-bsp" % fam
        d = os.path.join(ROOT, "crates", crate, "src")
        rows, doc = routes_of(fam)
        io.open(os.path.join(d, "pins.rs"), "w", encoding="utf-8", newline="\n").write(gen_pins(fam, rows, doc))
        pair = first_pa_pair(rows)
        print(crate, "pins.rs regenerated | uart pair:", pair)
