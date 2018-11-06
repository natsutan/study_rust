import glob
import matplotlib.pyplot as plt
import seaborn


def plot_graph(fname):
    y_data = []
    for l in open(fname).readlines():
        words = l.split()
        if words[0] == 'cost':
            print(l)
            break
        y_data.append(int(words[2]))

    x_data = range(1, len(y_data) + 1)

    plt.plot(x_data, y_data)
    plt.ylim(0, max(y_data) + 2)
    plt.savefig(fname+".png")


def plot_graphs(fname):
    files = glob.glob(fname)
#    plt.hold(True)
    for file in files:
        y_data = []
        for l in open(file).readlines():
            words = l.split()
            if words[0] == 'cost':
                break
            y_data.append(int(words[2]))

        x_data = range(1, len(y_data) + 1)

        plt.plot(x_data, y_data, '-')
    plt.ylim(0, max(y_data) + 2)
    plt.xlim(0, 50)
    plt.ylabel("cost")
    plt.savefig("sa.png")




#plot_graph('result1.txt')
plot_graphs('*.txt')

