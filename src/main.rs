fn main() {
    if let Ok(tokens) = bracoxide::tokenizer::tokenize("/etc/libvirt/hooks/{qemu,qemu.d/win10/{prepare/begin/{10-asusd-vfio,20-reserve-hugepages,30-set-governor}.sh,release/end/{10-release-hugepages,30-restore-governor,40-asusd-integrated,60-kill-looking-glass}.sh}}") {
        if let Ok(node) = bracoxide::parser::parse(&tokens) {
            println!("{:#?}", bracoxide::expand(&node));
        }
    }
}
