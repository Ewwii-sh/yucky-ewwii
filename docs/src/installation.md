# Installing Yucky Ewwii

To install and setup yucky-ewwii, go to the [releases page](https://github.com/Ewwii-sh/yucky-ewwii/releases), and grab the `libyucky_ewwii.so` file from the **appropriate** release. Choosing the appropriate release is easy. If you use ewwii v0.6.0, then you must download the file from the 0.6.0.x release. If you use the latest version of ewwii, then you can just download the file from latest release.

Once you have the `libyucky_ewwii.so` file, go to the configuration directory and create a directory named **"plugins"**. And just move the `libyucky_ewwii.so` file in here.

**That's it!** ewwii will now automatically loading it!

> **NOTE:**
>
> Using multiple language plugins like `yucky-ewwii` together will cause conflicts.
> Ensure that `yucky-ewwii` is the only language plugin that is present inside 
> the `plugins/` directory.
>
> Additionally, `yucky-ewwii` and similar language plugins will disable `rhai` entirely. 
> Meaning, configuration inside `ewwii.rhai` will be ignored.