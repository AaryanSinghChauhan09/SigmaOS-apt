use sigmaos_apt::{AptComponent, AptSource, Config, SourceList, Component};

#[test]
fn test_config_default_aaa() {
    let c = Config::default();
    assert!(c.enabled);
    assert_eq!(c.update_interval_hours, 24);
}

#[test]
fn test_apt_source_line_aaa() {
    let src = AptSource::new("http://archive.ubuntu.com/ubuntu", "jammy", &["main", "universe"]);
    let line = src.line();
    assert!(line.starts_with("deb http://archive.ubuntu.com/ubuntu jammy"));
    assert!(line.contains("main"));
}

#[test]
fn test_source_list_count_aaa() {
    let mut list = SourceList::new();
    list.add(AptSource::new("http://example.com", "stable", &["main"]));
    list.add(AptSource::new("http://example.com", "updates", &["main"]));
    assert_eq!(list.enabled_count(), 2);
}

#[test]
fn test_source_list_disabled_not_counted_aaa() {
    let mut list = SourceList::new();
    let mut src = AptSource::new("http://example.com", "stable", &["main"]);
    src.enabled = false;
    list.add(src);
    assert_eq!(list.enabled_count(), 0);
}

#[test]
fn test_component_lifecycle_aaa() {
    let mut c = AptComponent::new(Config::default());
    assert!(c.initialize().is_ok());
    assert!(c.health_check().is_ok());
    assert!(c.shutdown().is_ok());
}

#[test]
fn test_component_name_aaa() {
    let c = AptComponent::new(Config::default());
    assert_eq!(c.name(), "sigmaos-apt");
}
