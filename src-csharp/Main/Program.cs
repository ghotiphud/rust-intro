using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Threading;

namespace Main
{
    class Program
    {
        static void Main(string[] args)
        {
            var list = new List<int> { 1, 2, 3 };

            DefinitelyDoesntModify(list);

            Console.WriteLine(string.Join(", ", list));
            Console.ReadLine();
        }

        static void DataRace() {
            var i = 0;

            var threads = new List<Thread>();

            for (var j = 0; j < 10; j++)
            {
                var t = new Thread(() => {
                    var z = i;

                    Thread.Sleep(1);  // Simulate some quick work.

                    i = z + 1;

                    Console.WriteLine("z: {0}, i:{1}", z, i);
                });

                t.Start();
            }

            Thread.Sleep(1000);

            Console.WriteLine(i);
            Console.ReadLine();
        }

        static void Hello()
        {
            Console.WriteLine("Hello World!");
        }

        static void DefinitelyDoesntModify(List<int> l)
        {
            l.Add(50);
        }
    }
}
