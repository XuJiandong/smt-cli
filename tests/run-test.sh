
set -e
CLI=../target/debug/smt-cli
OUT=outputs
cd "$(dirname "${BASH_SOURCE[0]}")"
cd ..
cargo build
cd tests

${CLI} --hex --include="0|1|2" --kvpair 0xe8c0265680a02b680b6cbc880348f062b825b28e237da7169aded4bcac0a04e5 0x2ca41595841e46ce8e74ad749e5c3f1d17202150f99c3d8631233ebdd19b19eb 0x381dc5391dab099da5e28acd1ad859a051cf18ace804d037f12819c6fbc0e18b 0x9158ce9b0e11dd150ba2ae5d55c1db04b1c5986ec626f2e38a93fe8ad0b2923b 0xa9bb945be71f0bd2757d33d2465b6387383da42f321072e47472f0c9c7428a8a 0xa939a47335f777eac4c40fbc0970e25f832a24e1d55adc45a7b76d63fe364e82 \
> ${OUT}/multi_3.txt
${CLI} --include "0|1" 11 22 > ${OUT}/simple_include.txt
${CLI} --exclude "11|22" 111 222 > ${OUT}/simple_exclude.txt
${CLI} --include "0" 11 22 > ${OUT}/simple_include_0.txt
${CLI} --include "1" 11 22 > ${OUT}/simple_include_1.txt
git diff --exit-code outputs/*.txt
