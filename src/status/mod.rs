mod specialhi;
mod specialhihold;
mod specialhiend;

mod attachwall;

mod grabs;

pub fn install() {
    specialhi::install();
    specialhihold::install();
    specialhiend::install();

    attachwall::install();

    grabs::install();
}