# Bazel Build Rules for CPM Polyglot Workspaces

def cpm_dependency(name, package_spec):
    """
    Bazel rule to fetch and link polyglot CPM dependencies.
    """
    native.genrule(
        name = name,
        outs = [name + ".tar.gz"],
        cmd = "cpm add " + package_spec,
    )
