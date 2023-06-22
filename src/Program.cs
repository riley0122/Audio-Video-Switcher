using FFMpegCore;

namespace avSwitcher{
    class Program{
        static void Main(string[] args){
            string filename = args[0];
            string output = filename + "-inverted.mp4";

            FFMpegArguments
                .FromFileInput(filename)
                .OutputToFile(output, false, options => options
                    .WithCustomArgument("-map 0:a:0 -map 0:v:1"))
                .ProcessSynchronously();

            Console.WriteLine("switched");
        }
    }
}