import pandas as pd
from pandas import DataFrame, Series
import matplotlib.pyplot as plt
import os
import sys
from typing import Dict, List


def generate(figure: str, label: str, dfs: Dict[str, DataFrame], lines: List[Dict], data_dir: str) -> None:
    fig, ax = plt.subplots()
    ax.set(xlabel='time step', ylabel=label)
    for line in lines:
        alpha = line.get('alpha')
        c = line.get('c')
        eps = line.get('epsilon')
        if alpha is not None:
            alpha = str(alpha)
            series = dfs['alpha=' + alpha][label]
            ax.plot(series.index, series.values,
                    color=line['color'], label=f"alpha={alpha}", linewidth=0.6)
        elif c is not None:
            c = str(c)
            series = dfs['c=' + c][label]
            ax.plot(series.index, series.values,
                    color=line['color'], label=f"c={c}", linewidth=0.6)
        elif eps is not None:
            eps = str(eps)
            series = dfs['eps=' + eps][label]
            ax.plot(series.index, series.values,
                    color=line['color'], label=f"ε={eps}", linewidth=0.6)

    ax.legend()
    plt.title(figure)
    outfile = os.path.join(data_dir, label + '.svg')
    fig.savefig(outfile)
    plt.close()


def plot(figure, lines, base_dir: str) -> None:
    dfs = {}
    for line in lines:
        alpha = line.get('alpha')
        c = line.get('c')
        eps = line.get('epsilon')
        if alpha is not None:
            alpha = str(alpha)
            filename = os.path.join(base_dir, f"alpha={alpha}.csv")
            dfs['alpha=' + alpha] = pd.read_csv(filename, sep=',', header='infer')
        elif c is not None:
            c = str(c)
            filename = os.path.join(base_dir, f"c={c}.csv")
            dfs['c=' + c] = pd.read_csv(filename, sep=',', header='infer')
        elif eps is not None:
            eps = str(eps)
            filename = os.path.join(base_dir, f"eps={eps}.csv")
            dfs['eps=' + eps] = pd.read_csv(filename, sep=',', header='infer')
    out_dir = os.path.join(base_dir, figure)
    os.makedirs(out_dir, exist_ok=True)
    generate(figure, 'reward', dfs, lines, out_dir)
    generate(figure, 'optimal', dfs, lines, out_dir)


if __name__ == "__main__":
    data_dir = sys.argv[1]
    kind = sys.argv[2]
    if kind == 'fig_2_2':
        plot('fig_2_2', [
            {"epsilon": 0, "color": "green"},
            {"epsilon": 0.1, "color": "blue"},
            {"epsilon": 0.01, "color": "red"},
        ], data_dir)
    elif kind == 'fig_2_4':
        plot('fig_2_4', [
            {"epsilon": 0.1, "color": "gray"},
            {"c": 2, "color": "blue"},
        ], data_dir)
    elif kind == 'fig_2_5':
        plot('fig_2_5', [
            {"alpha": 0.1, "color": "red"},
            {"alpha": 0.4, "color": "green"},
        ], data_dir)

